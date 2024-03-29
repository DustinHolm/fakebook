use async_graphql::{Context, InputObject, Object, ID};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use time::OffsetDateTime;
use tracing::instrument;

use crate::{
    domain::{
        app_user::AppUser,
        comment::Comment,
        errors::GqlError,
        relay_meta::{paginate, AppConnection},
    },
    infrastructure::db::Loaders,
};

use super::domain::{Post, SUFFIX};

#[Object]
impl Post {
    pub async fn id(&self) -> ID {
        let combined = self.post_id.to_string() + SUFFIX;

        ID(URL_SAFE.encode(combined))
    }

    #[instrument(skip_all, err)]
    #[graphql(complexity = 3)]
    async fn author(&self, ctx: &Context<'_>) -> Result<AppUser, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        loaders
            .app_user
            .load_one(self.author)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected author, got None".to_string()))
    }

    async fn created_on(&self) -> OffsetDateTime {
        self.created_on
    }

    async fn content(&self) -> &str {
        &self.content
    }

    #[instrument(skip_all, err)]
    #[graphql(
        complexity = "first.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity 
        + last.unwrap_or(0).try_into().unwrap_or(usize::MAX) * child_complexity"
    )]
    async fn comments(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<AppConnection<Comment>, GqlError> {
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        let comments = loaders
            .comments_of_post
            .load_one(self.post_id)
            .await
            .map_err(|_| GqlError::DbLoad)?
            .ok_or_else(|| GqlError::InvalidState("Expected empty vec, got None".to_string()))?;

        let connection = paginate(after, before, first, last, comments)
            .await
            .map_err(|_| GqlError::InternalData)?;

        Ok(connection)
    }
}

#[derive(Debug, InputObject)]
pub struct PostInput {
    pub(in crate::domain) content: String,
}
