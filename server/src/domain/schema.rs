use async_graphql::{
    connection::{Edge, EmptyFields},
    Context, Object, ID,
};
use tracing::instrument;

use crate::{
    errors::{mapping::MappingError, query::QueryError},
    infrastructure::db::{Loaders, Saver},
};

use super::{
    app_user::{AppUser, AppUserInput},
    comment::{Comment, CommentInput},
    post::{Post, PostInput},
    relay_meta::{AppCursor, CanDecodeId, Node},
    ValidInput as _,
};

pub struct RootQuery;

#[Object]
impl RootQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Node, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        if let Ok(inner_id) = AppUser::decode(&id) {
            let user = loaders
                .app_user
                .load_one(inner_id)
                .await?
                .ok_or_else(QueryError::not_found)?;

            return Ok(Node::AppUser(user));
        }

        if let Ok(inner_id) = Comment::decode(&id) {
            let comment = loaders
                .comment
                .load_one(inner_id)
                .await?
                .ok_or_else(QueryError::not_found)?;

            return Ok(Node::Comment(comment));
        }

        if let Ok(inner_id) = Post::decode(&id) {
            let post = loaders
                .post
                .load_one(inner_id)
                .await?
                .ok_or_else(QueryError::not_found)?;

            return Ok(Node::Post(post));
        }

        Err(QueryError::invalid_input(
            "Given id did not match any available types".into(),
        ))
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<AppUser, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        let inner_id = AppUser::decode(&id)
            .map_err(|e: MappingError| QueryError::invalid_input(e.to_string()))?;

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

        user.validate()?;

        Ok(saver.save_user(&user).await?)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        post: PostInput,
    ) -> Result<Edge<AppCursor, Post, EmptyFields>, QueryError> {
        let saver = ctx
            .data::<Saver>()
            .map_err(|e| QueryError::internal(e.message))?;

        post.validate()?;

        let saved = saver.save_post(&post).await?;

        Ok(Edge::new(AppCursor(saved.post_id), saved))
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        comment: CommentInput,
    ) -> Result<Edge<AppCursor, Comment, EmptyFields>, QueryError> {
        let saver = ctx
            .data::<Saver>()
            .map_err(|e| QueryError::internal(e.message))?;

        comment.validate()?;

        let saved = saver.save_comment(&comment).await?;

        Ok(Edge::new(AppCursor(saved.comment_id), saved))
    }
}
