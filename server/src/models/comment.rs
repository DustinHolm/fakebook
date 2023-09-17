use async_graphql::{Context, Object, ID};
use time::OffsetDateTime;

use crate::{errors::query::QueryError, infrastructure::db::Loaders};

use super::{app_user::AppUser, post::Post};

pub struct Comment {
    comment_id: i32,
    referenced_post: i32,
    author: i32,
    created_on: OffsetDateTime,
    content: String,
}

#[Object]
impl Comment {
    async fn id(&self) -> ID {
        ID(self.comment_id.to_string())
    }

    async fn referenced_post(&self, ctx: &Context<'_>) -> Result<Post, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        loaders
            .post
            .load_one(self.referenced_post)
            .await?
            .ok_or(QueryError::not_found())
    }

    async fn author(&self, ctx: &Context<'_>) -> Result<AppUser, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        loaders
            .app_user
            .load_one(self.author)
            .await?
            .ok_or(QueryError::not_found())
    }

    async fn created_on(&self) -> OffsetDateTime {
        self.created_on
    }

    async fn content(&self) -> &str {
        &self.content
    }
}
