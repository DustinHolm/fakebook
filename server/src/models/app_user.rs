use std::collections::HashMap;

use async_graphql::{
    dataloader::{DataLoader, Loader},
    Context, Object,
};
use axum::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::{
    types::{to_sql_checked, ToSql},
    Row,
};
use tracing::instrument;

use crate::errors::{mapping::MappingError, query::QueryError};

#[derive(Clone)]
pub struct AppUser {
    user_id: i32,
    first_name: String,
    last_name: String,
}

#[Object]
impl AppUser {
    pub async fn user_id(&self) -> i32 {
        self.user_id
    }

    pub async fn first_name(&self) -> &str {
        &self.first_name
    }

    pub async fn last_name(&self) -> &str {
        &self.last_name
    }

    #[instrument(skip(self, ctx), err(Debug))]
    pub async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<AppUser>, QueryError> {
        let friend_loader = ctx
            .data::<DataLoader<FriendIdLoader>>()
            .map_err(|e| QueryError::internal(e.message))?;

        let user_loader = ctx
            .data::<DataLoader<AppUserLoader>>()
            .map_err(|e| QueryError::internal(e.message))?;

        let friend_ids = friend_loader
            .load_one(self.user_id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        let users = user_loader
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
    type Error = MappingError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await?;

        let rows = db
            .query("SELECT * FROM app_user WHERE user_id = ANY ($1)", &[&ids])
            .await
            .map_err(MappingError::from_db)?;

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
    type Error = MappingError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await?;
        let mut result_map = HashMap::with_capacity(ids.len());

        for id in ids {
            let rows = db
                .query(
                    r#"
                        SELECT user_id_a
                        FROM user_relation
                        WHERE user_id_b = $1
                        UNION
                        SELECT user_id_b
                        FROM user_relation
                        WHERE user_id_a = $1
                    "#,
                    &[id],
                )
                .await
                .map_err(MappingError::from_db)?;

            let friend_ids = rows
                .into_iter()
                .map(|row| row.try_get(0))
                .collect::<Result<Vec<i32>, tokio_postgres::Error>>()
                .map_err(MappingError::from_db)?;

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
            user_id: value.try_get("user_id").map_err(MappingError::from_db)?,
            first_name: value.try_get("first_name").map_err(MappingError::from_db)?,
            last_name: value.try_get("last_name").map_err(MappingError::from_db)?,
        })
    }
}
