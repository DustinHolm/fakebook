use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use axum::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    domain::{db_id::DbId, errors::DbError},
    infrastructure::db::Saver,
};

use super::domain::AppUser;

pub struct AppUserLoader {
    pool: Pool,
}

impl AppUserLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<DbId> for AppUserLoader {
    type Value = AppUser;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;

        let stmt = db
            .prepare_cached("SELECT * FROM app_user WHERE user_id = ANY ($1)")
            .await
            .map_err(DbError::statement)?;

        let rows = db.query(&stmt, &[&ids]).await.map_err(DbError::statement)?;

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
impl Loader<DbId> for FriendIdLoader {
    type Value = Vec<DbId>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;

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
            .await
            .map_err(DbError::statement)?;

        let relations = db
            .query(&stmt, &[&ids])
            .await
            .map_err(DbError::statement)?
            .into_iter()
            .map(|row| {
                Ok((
                    row.try_get(0).map_err(DbError::mapping)?,
                    row.try_get(1).map_err(DbError::mapping)?,
                ))
            })
            .collect::<Result<Vec<(DbId, DbId)>, DbError>>()?;

        let result_map = ids
            .iter()
            .map(|id| {
                let friends: Vec<DbId> = relations
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

impl Saver {
    #[instrument(skip(self), err)]
    pub async fn save_user(&self, first_name: &str, last_name: &str) -> Result<AppUser, DbError> {
        let db = self.pool.get().await?;

        let stmt = db
            .prepare_cached(
                r"
                    INSERT INTO app_user (first_name, last_name)
                    VALUES ($1, $2)
                    RETURNING *
                ",
            )
            .await
            .map_err(DbError::statement)?;

        let row = db
            .query_one(&stmt, &[&first_name, &last_name])
            .await
            .map_err(DbError::statement)?;

        row.try_into()
    }

    #[instrument(skip(self), err)]
    pub async fn add_friend(&self, user: &DbId, friend: &DbId) -> Result<(), DbError> {
        let db = self.pool.get().await?;

        let stmt = db
            .prepare_cached(
                r"
                    INSERT INTO user_relation (user_id_a, user_id_b)
                    VALUES ($1, $2)
                    ON CONFLICT ON CONSTRAINT user_relation_pkey
                    DO NOTHING
                ",
            )
            .await
            .map_err(DbError::statement)?;

        let mut users = [user, friend];
        users.sort_unstable();

        db.execute(&stmt, &[&users[0], &users[1]])
            .await
            .map_err(DbError::statement)?;

        Ok(())
    }
}

impl TryFrom<Row> for AppUser {
    type Error = DbError;

    #[instrument(err)]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(AppUser {
            user_id: value.try_get("user_id").map_err(DbError::mapping)?,
            first_name: value.try_get("first_name").map_err(DbError::mapping)?,
            last_name: value.try_get("last_name").map_err(DbError::mapping)?,
        })
    }
}
