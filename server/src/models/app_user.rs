use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Context, InputObject, Object, ID};
use axum::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    errors::{db::DbError, mapping::MappingError, query::QueryError},
    infrastructure::db::{Loaders, Saver},
};

use super::post::Post;

#[derive(Clone)]
pub struct AppUser {
    user_id: i32,
    first_name: String,
    last_name: String,
}

#[Object]
impl AppUser {
    pub async fn id(&self) -> ID {
        ID(self.user_id.to_string())
    }

    pub async fn first_name(&self) -> &str {
        &self.first_name
    }

    pub async fn last_name(&self) -> &str {
        &self.last_name
    }

    #[instrument(skip_all, err(Debug))]
    pub async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<AppUser>, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        let friend_ids = loaders
            .friend_id
            .load_one(self.user_id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        let users = loaders
            .app_user
            .load_many(friend_ids)
            .await?
            .into_values()
            .collect();

        Ok(users)
    }

    #[instrument(skip_all, err(Debug))]
    pub async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        let posts = loaders
            .posts_of_author
            .load_one(self.user_id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        Ok(posts)
    }
}

pub struct AppUserLoader {
    pool: Pool,
}

impl AppUserLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<i32> for AppUserLoader {
    type Value = AppUser;
    type Error = DbError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(DbError::connection)?;
        let stmt = db
            .prepare_cached("SELECT * FROM app_user WHERE user_id = ANY ($1)")
            .await?;

        let rows = db.query(&stmt, &[&ids]).await?;

        rows.into_iter()
            .map(|row| {
                let user: AppUser = row.try_into()?;
                Ok((user.user_id, user))
            })
            .collect()
    }
}

pub struct FriendIdLoader {
    pool: Pool,
}

impl FriendIdLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<i32> for FriendIdLoader {
    type Value = Vec<i32>;
    type Error = DbError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(DbError::connection)?;
        let stmt = db
            .prepare_cached(
                r#"
                    SELECT user_id_a, user_id_b
                    FROM user_relation
                    WHERE user_id_b = ANY($1)
                    UNION
                    SELECT user_id_a, user_id_b
                    FROM user_relation
                    WHERE user_id_a = ANY($1)
                "#,
            )
            .await?;

        let relations = db
            .query(&stmt, &[&ids])
            .await?
            .into_iter()
            .map(|row| {
                Ok((
                    row.try_get(0).map_err(MappingError::db)?,
                    row.try_get(1).map_err(MappingError::db)?,
                ))
            })
            .collect::<Result<Vec<(i32, i32)>, DbError>>()?;

        let result_map = ids
            .iter()
            .map(|id| {
                let friends: Vec<i32> = relations
                    .iter()
                    .filter_map(|rel| {
                        if &rel.0 == id {
                            Some(rel.1)
                        } else if &rel.1 == id {
                            Some(rel.0)
                        } else {
                            None
                        }
                    })
                    .collect();

                (*id, friends)
            })
            .collect();

        Ok(result_map)
    }
}

#[derive(InputObject, Debug)]
pub struct AppUserInput {
    pub first_name: String,
    pub last_name: String,
}

#[async_trait]
impl Saver for AppUserInput {
    type Saved = AppUser;
    type Error = DbError;

    #[instrument(skip(pool), err(Debug))]
    async fn save(&self, pool: &Pool) -> Result<Self::Saved, Self::Error> {
        let db = pool.get().await?;

        let stmt = db
            .prepare_cached(
                r"
                    INSERT INTO app_user (first_name, last_name)
                    VALUES ($1, $2)
                    RETURNING *
                ",
            )
            .await?;

        let row = db
            .query_one(&stmt, &[&self.first_name, &self.last_name])
            .await?;

        Ok(row.try_into()?)
    }
}

impl TryFrom<Row> for AppUser {
    type Error = MappingError;

    #[instrument(err(Debug))]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(AppUser {
            user_id: value.try_get("user_id").map_err(MappingError::db)?,
            first_name: value.try_get("first_name").map_err(MappingError::db)?,
            last_name: value.try_get("last_name").map_err(MappingError::db)?,
        })
    }
}
