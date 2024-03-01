use async_graphql::{Context, InputObject, Object, ID};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use time::OffsetDateTime;
use tracing::instrument;

use crate::{
    domain::{app_user::AppUser, errors::GqlError, post::Post},
    infrastructure::db::Loaders,
};

use super::{domain::SUFFIX, Comment};

#[Object]
impl Comment {
    pub async fn id(&self) -> ID {
        let combined = self.comment_id.to_string() + SUFFIX;

        ID(URL_SAFE.encode(combined))
    }

    #[instrument(skip_all, err)]
    async fn referenced_post(&self, ctx: &Context<'_>) -> Result<Post, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        loaders
            .post
            .load_one(self.referenced_post)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))
    }

    #[instrument(skip_all, err)]
    async fn author(&self, ctx: &Context<'_>) -> Result<AppUser, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        loaders
            .app_user
            .load_one(self.author)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))
    }

    async fn created_on(&self) -> OffsetDateTime {
        self.created_on
    }

    async fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, InputObject)]
pub struct CommentInput {
    pub(in crate::domain) author: ID,
    pub(in crate::domain) content: String,
    pub(in crate::domain) referenced_post: ID,
}
