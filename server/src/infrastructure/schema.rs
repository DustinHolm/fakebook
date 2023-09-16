use async_graphql::{dataloader::DataLoader, Context, EmptyMutation, EmptySubscription, Object};
use deadpool_postgres::Pool;
use tokio::spawn;
use tracing::instrument;

use crate::{
    errors::query::QueryError,
    models::app_user::{AppUser, AppUserLoader, FriendIdLoader},
};

pub fn build_with_dataloaders(pool: Pool) -> Schema {
    let app_user_loader = DataLoader::new(AppUserLoader::new(pool.clone()), spawn);
    let friend_id_loader = DataLoader::new(FriendIdLoader::new(pool), spawn);

    Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(app_user_loader)
        .data(friend_id_loader)
        .finish()
}

pub type Schema = async_graphql::Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<AppUser, QueryError> {
        let loader = ctx
            .data::<DataLoader<AppUserLoader>>()
            .map_err(|e| QueryError::internal(e.message))?;

        let user = loader
            .load_one(id)
            .await?
            .ok_or_else(QueryError::not_found)?;

        Ok(user)
    }
}
