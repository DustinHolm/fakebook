use std::num::ParseIntError;

use async_graphql::{Context, Object, ID};
use deadpool_postgres::Pool;
use tracing::instrument;

use crate::{
    errors::query::QueryError,
    infrastructure::db::{Loaders, Saver},
};

use super::app_user::{AppUser, AppUserInput};

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<AppUser, QueryError> {
        let inner_id: i32 = id
            .try_into()
            .map_err(|e: ParseIntError| QueryError::invalid_input(e.to_string()))?;

        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        let user = loaders
            .app_user
            .load_one(inner_id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        Ok(user)
    }
}

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        user: AppUserInput,
    ) -> Result<AppUser, QueryError> {
        let pool = ctx
            .data::<Pool>()
            .map_err(|e| QueryError::internal(e.message))?;

        Ok(user.save(pool).await?)
    }
}
