use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Context, Object, ID};
use axum::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    errors::{loader::LoaderError, mapping::MappingError, query::QueryError},
    infrastructure::db::Loaders,
};

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

    #[instrument(skip(self, ctx), err(Debug))]
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
    type Error = LoaderError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(LoaderError::connection)?;

        let rows = db
            .query("SELECT * FROM app_user WHERE user_id = ANY ($1)", &[&ids])
            .await?;

        let result = rows
            .into_iter()
            .map(|row| {
                let user: AppUser = row.try_into()?;
                Ok((user.user_id, user))
            })
            .collect();

        result
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
    type Error = LoaderError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(LoaderError::connection)?;
        let mut result_map = HashMap::with_capacity(ids.len());
        let statement = db
            .prepare(
                r#"
                    SELECT user_id_a
                    FROM user_relation
                    WHERE user_id_b = $1
                    UNION
                    SELECT user_id_b
                    FROM user_relation
                    WHERE user_id_a = $1
                "#,
            )
            .await?;

        for id in ids {
            let rows = db.query(&statement, &[id]).await?;

            let friend_ids = rows
                .into_iter()
                .map(|row| row.try_get(0))
                .collect::<Result<Vec<i32>, tokio_postgres::Error>>()
                .map_err(MappingError::db)?;

            result_map.insert(*id, friend_ids);
        }

        Ok(result_map)
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
