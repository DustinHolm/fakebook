use async_graphql::{Context, Object};
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::instrument;

use crate::errors::{mapping::MappingError, query::QueryError};

pub struct AppUser {
    user_id: i32,
    first_name: String,
    last_name: String,
}

impl TryFrom<Row> for AppUser {
    type Error = MappingError;

    #[instrument(err())]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(AppUser {
            user_id: value.try_get("user_id").map_err(MappingError::from_db)?,
            first_name: value.try_get("first_name").map_err(MappingError::from_db)?,
            last_name: value.try_get("last_name").map_err(MappingError::from_db)?,
        })
    }
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

    pub async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<AppUser>, QueryError> {
        let db = ctx.data::<Pool>().unwrap().get().await?;

        let rows = db
            .query(
                r#"
                    SELECT *
                    FROM app_user
                    WHERE user_id IN (
                        SELECT user_id_a
                        FROM user_relation
                        WHERE user_id_b = $1
                        UNION
                        SELECT user_id_b
                        FROM user_relation
                        WHERE user_id_a = $1
                    )
                "#,
                &[&self.user_id],
            )
            .await?;

        let users = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<AppUser>, MappingError>>()?;

        Ok(users)
    }
}
