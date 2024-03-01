use async_graphql::{
    connection::{Edge, EmptyFields},
    Context, Object,
};
use tracing::instrument;

use crate::{
    domain::{
        app_user::{AddFriendInput, AppUser, AppUserInput},
        comment::{Comment, CommentInput},
        db_id::CanDecodeId,
        errors::GqlError,
        post::{Post, PostInput},
        relay_meta::AppCursor,
    },
    infrastructure::db::{Loaders, Saver},
};

pub struct RootMutation;

#[Object]
impl RootMutation {
    #[instrument(skip(self, ctx), err)]
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: AppUserInput,
    ) -> Result<AppUser, GqlError> {
        let saver = ctx.data::<Saver>().map_err(|_| GqlError::InternalData)?;

        saver
            .save_user(&input.first_name, &input.last_name)
            .await
            .map_err(|_| GqlError::DbSave)
    }

    #[instrument(skip(self, ctx), err)]
    async fn add_friend(
        &self,
        ctx: &Context<'_>,
        input: AddFriendInput,
    ) -> Result<AppUser, GqlError> {
        let saver = ctx.data::<Saver>().map_err(|_| GqlError::InternalData)?;
        let loaders = ctx.data::<Loaders>().map_err(|_| GqlError::InternalData)?;

        let user_id =
            AppUser::decode(&input.user).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;
        let friend_id =
            AppUser::decode(&input.friend).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let mut users = loaders
            .app_user
            .load_many([user_id, friend_id])
            .await
            .map_err(|_| GqlError::DbLoad)?;

        let user = users.remove(&user_id).ok_or_else(|| {
            GqlError::InvalidState("This was requested for a user that does not exist".to_string())
        })?;

        users.get(&friend_id).ok_or_else(|| {
            GqlError::InvalidState(
                "This was requested for a friend that does not exist".to_string(),
            )
        })?;

        saver
            .add_friend(&user_id, &friend_id)
            .await
            .map_err(|_| GqlError::DbSave)?;

        Ok(user)
    }

    #[instrument(skip(self, ctx), err)]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: PostInput,
    ) -> Result<Edge<AppCursor, Post, EmptyFields>, GqlError> {
        let saver = ctx.data::<Saver>().map_err(|_| GqlError::InternalData)?;

        let author =
            AppUser::decode(&input.author).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let saved = saver
            .save_post(&author, &input.content)
            .await
            .map_err(|_| GqlError::DbSave)?;

        Ok(Edge::new(AppCursor(saved.post_id), saved))
    }

    #[instrument(skip(self, ctx), err)]
    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        input: CommentInput,
    ) -> Result<Edge<AppCursor, Comment, EmptyFields>, GqlError> {
        let saver = ctx.data::<Saver>().map_err(|_| GqlError::InternalData)?;

        let author_id =
            AppUser::decode(&input.author).map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let referenced_post_id = Post::decode(&input.referenced_post)
            .map_err(|e| GqlError::InvalidRequest(e.to_string()))?;

        let saved = saver
            .save_comment(&author_id, &referenced_post_id, &input.content)
            .await
            .map_err(|_| GqlError::DbSave)?;

        Ok(Edge::new(AppCursor(saved.comment_id), saved))
    }
}
