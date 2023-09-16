use async_graphql::{dataloader::DataLoader, Context, EmptyMutation, EmptySubscription, Object};
use deadpool_postgres::Pool;
use tokio::spawn;
use tracing::instrument;

use crate::{
    errors::query::QueryError,
    models::app_user::{AppUser, AppUserLoader, FriendIdLoader},
};

pub fn build_with_dataloaders(pool: Pool) -> Schema {
    Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub type Schema = async_graphql::Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<AppUser, QueryError> {
        let db = ctx
            .data::<Pool>()
            .map_err(|e| QueryError::internal(e.message))?
            .get()
            .await?;

        let user = db
            .query_opt("SELECT * FROM app_user WHERE user_id = $1", &[&id])
            .await?
            .ok_or_else(QueryError::not_found)?
            .try_into()?;

        Ok(user)
    }
}
