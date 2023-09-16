use async_graphql::{Context, Object};
use tracing::instrument;

use crate::{errors::query::QueryError, infrastructure::db::AppLoader};

use super::app_user::AppUser;

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<AppUser, QueryError> {
        let loader = ctx
            .data::<AppLoader>()
            .map_err(|e| QueryError::internal(e.message))?;

        let user = loader
            .app_user
            .load_one(id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        Ok(user)
    }
}
