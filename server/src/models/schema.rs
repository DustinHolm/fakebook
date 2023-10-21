use std::num::ParseIntError;

use async_graphql::{Context, Object, ID};
use tracing::instrument;

use crate::{
    errors::query::QueryError,
    infrastructure::db::{Loaders, Saver},
};

use super::{
    app_user::{AppUser, AppUserInput},
    comment::{Comment, CommentInput},
    post::{Post, PostInput},
};

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
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        user: AppUserInput,
    ) -> Result<AppUser, QueryError> {
        let saver = ctx
            .data::<Saver>()
            .map_err(|e| QueryError::internal(e.message))?;

        Ok(saver.save_user(&user).await?)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_post(&self, ctx: &Context<'_>, post: PostInput) -> Result<Post, QueryError> {
        let saver = ctx
            .data::<Saver>()
            .map_err(|e| QueryError::internal(e.message))?;

        Ok(saver.save_post(&post).await?)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        comment: CommentInput,
    ) -> Result<Comment, QueryError> {
        let saver = ctx
            .data::<Saver>()
            .map_err(|e| QueryError::internal(e.message))?;

        Ok(saver.save_comment(&comment).await?)
    }
}
