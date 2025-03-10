use async_graphql::{Context, Object, ID};
use tracing::instrument;

use crate::{
    domain::{
        app_user::AppUser,
        comment::Comment,
        db_id::{CanDecodeId as _, DbId},
        errors::GqlError,
        post::Post,
        relay_meta::Node,
        viewer::Viewer,
    },
    infrastructure::db::Loaders,
};

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err)]
    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Node, GqlError> {
        let loaders = ctx.data::<Loaders>()?;

        if let Ok(inner_id) = AppUser::decode(&id) {
            let user = loaders
                .app_user
                .load_one(inner_id)
                .await
                .map_err(|_| GqlError::DbLoad)?
                .ok_or_else(|| {
                    GqlError::InvalidState("Expected empty vec, got None".to_string())
                })?;

            return Ok(Node::AppUser(user));
        }

        if let Ok(inner_id) = Comment::decode(&id) {
            let comment = loaders
                .comment
                .load_one(inner_id)
                .await
                .map_err(|_| GqlError::DbLoad)?
                .ok_or_else(|| {
                    GqlError::InvalidState("Expected empty vec, got None".to_string())
                })?;
            return Ok(Node::Comment(comment));
        }

        if let Ok(inner_id) = Post::decode(&id) {
            let post = loaders
                .post
                .load_one(inner_id)
                .await
                .map_err(|_| GqlError::DbLoad)?
                .ok_or_else(|| {
                    GqlError::InvalidState("Expected empty vec, got None".to_string())
                })?;

            return Ok(Node::Post(post));
        }

        Err(GqlError::InvalidRequest(
            "Given id did not match any available types".to_string(),
        ))
    }

    #[instrument(skip(self, ctx), err)]
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<AppUser, GqlError> {
        let loaders = ctx.data::<Loaders>()?;

        let inner_id = AppUser::decode(&id).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let user = loaders
            .app_user
            .load_one(inner_id)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        Ok(user)
    }

    #[instrument(skip(self, ctx), err)]
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Viewer, GqlError> {
        let loaders = ctx.data::<Loaders>()?;

        let id = DbId::from(1); // Placeholder until we have auth

        let user = loaders
            .app_user
            .load_one(id)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        Ok(Viewer::new(user))
    }
}
