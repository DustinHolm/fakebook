use async_graphql::{Context, EmptyMutation, EmptySubscription, Object};
use deadpool_postgres::Pool;
use tracing::instrument;

use crate::errors::query::QueryError;

use super::app_user::AppUser;

pub type Schema = async_graphql::Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<AppUser, QueryError> {
        let db = ctx.data::<Pool>().unwrap().get().await?;

        let row = db
            .query_opt("SELECT * FROM app_user WHERE user_id = $1", &[&id])
            .await?
            .ok_or_else(QueryError::not_found)?;

        let user = row.try_into()?;

        Ok(user)
    }
}
