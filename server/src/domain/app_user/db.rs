use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use tokio_postgres::Row;
use tracing::{instrument, Level};

use crate::{
    domain::db_id::DbId,
    infrastructure::{db::Repo, DbError},
};

use super::domain::AppUser;

pub struct AppUserLoader {
    repo: Repo,
}

impl AppUserLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for AppUserLoader {
    type Value = AppUser;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        self.repo
            .query(
                "SELECT * FROM app_user WHERE user_id = ANY ($1)",
                &[&ids],
                |rows| {
                    rows.into_iter()
                        .map(|row| {
                            let user: AppUser = row.try_into()?;
                            Ok::<_, DbError>((user.user_id, user))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                },
            )
            .await
            .map_err(Arc::new)
    }
}

pub struct FriendIdLoader {
    repo: Repo,
}

impl FriendIdLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for FriendIdLoader {
    type Value = Vec<DbId>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let relations: Vec<(DbId, DbId)> = self
            .repo
            .query(
                r#"
                    SELECT user_id_a, user_id_b
                    FROM user_relation
                    WHERE user_id_b = ANY($1)
                    UNION
                    SELECT user_id_a, user_id_b
                    FROM user_relation
                    WHERE user_id_a = ANY($1)
                "#,
                &[&ids],
                |rows| {
                    rows.into_iter()
                        .map(|row| {
                            let user_id_a = row.try_get(0).map_err(DbError::mapping)?;
                            let user_id_b = row.try_get(1).map_err(DbError::mapping)?;
                            Ok::<_, DbError>((user_id_a, user_id_b))
                        })
                        .collect()
                },
            )
            .await?;

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

impl Repo {
    #[instrument(skip(self), err)]
    pub async fn save_user(&self, first_name: &str, last_name: &str) -> Result<AppUser, DbError> {
        self.query_one(
            r"
                INSERT INTO app_user (first_name, last_name)
                VALUES ($1, $2)
                RETURNING *
            ",
            &[&first_name, &last_name],
            |row| row.try_into(),
        )
        .await
    }

    #[instrument(skip(self), err)]
    pub async fn add_friend(&self, user: &DbId, friend: &DbId) -> Result<(), DbError> {
        let mut users = [user, friend];
        users.sort_unstable();

        self.execute(
            r"
                INSERT INTO user_relation (user_id_a, user_id_b)
                VALUES ($1, $2)
                ON CONFLICT ON CONSTRAINT user_relation_pkey
                DO NOTHING
            ",
            &[&users[0], &users[1]],
        )
        .await
    }
}

impl TryFrom<Row> for AppUser {
    type Error = DbError;

    #[instrument(level = Level::TRACE, err)]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(AppUser {
            user_id: value.try_get("user_id").map_err(DbError::mapping)?,
            first_name: value.try_get("first_name").map_err(DbError::mapping)?,
            last_name: value.try_get("last_name").map_err(DbError::mapping)?,
        })
    }
}
