use async_graphql::{
    dataloader::{DataLoader, HashMapCache},
    Context, Object,
};
use tracing::instrument;

use crate::errors::query::QueryError;

use super::app_user::{AppUser, AppUserLoader};

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<AppUser, QueryError> {
        let loader = ctx
            .data::<DataLoader<AppUserLoader, HashMapCache>>()
            .map_err(|e| QueryError::internal(e.message))?;

        let user = loader
            .load_one(id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        Ok(user)
    }
}
