use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Context, Object, ID};
use axum::async_trait;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    errors::{loader::LoaderError, mapping::MappingError, query::QueryError},
    infrastructure::db::Loaders,
};

use super::{app_user::AppUser, comment::Comment};

#[derive(Clone)]
pub struct Post {
    post_id: i32,
    author: i32,
    created_on: OffsetDateTime,
    content: String,
}

#[Object]
impl Post {
    async fn id(&self) -> ID {
        ID(self.post_id.to_string())
    }

    #[instrument(skip_all, err(Debug))]
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

    #[instrument(skip_all, err(Debug))]
    async fn comments(&self, ctx: &Context<'_>) -> Result<Vec<Comment>, QueryError> {
        let loaders = ctx
            .data::<Loaders>()
            .map_err(|e| QueryError::internal(e.message))?;

        loaders
            .comments_of_post
            .load_one(self.post_id)
            .await?
            .ok_or(QueryError::not_found())
    }
}

pub struct PostLoader {
    pool: Pool,
}

impl PostLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<i32> for PostLoader {
    type Value = Post;
    type Error = LoaderError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(LoaderError::connection)?;

        let rows = db
            .query("SELECT * FROM post WHERE post_id = ANY($1)", &[&ids])
            .await?;

        let result = rows
            .into_iter()
            .map(|row| {
                let post: Post = row.try_into()?;
                Ok((post.post_id, post))
            })
            .collect();

        result
    }
}

pub struct PostsOfAuthorLoader {
    pool: Pool,
}

impl PostsOfAuthorLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<i32> for PostsOfAuthorLoader {
    type Value = Vec<Post>;
    type Error = LoaderError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(LoaderError::connection)?;

        let rows = db
            .query("SELECT * FROM post WHERE author = ANY($1)", &[&ids])
            .await?;

        let mut result = HashMap::from_iter(ids.iter().map(|id| (*id, Vec::new())));

        for row in rows {
            let post: Post = row.try_into()?;
            result
                .entry(post.author)
                .and_modify(|e: &mut Vec<Post>| e.push(post));
        }

        Ok(result)
    }
}

impl TryFrom<Row> for Post {
    type Error = MappingError;

    #[instrument(err(Debug))]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Post {
            post_id: value.try_get("post_id").map_err(MappingError::db)?,
            author: value.try_get("author").map_err(MappingError::db)?,
            created_on: value.try_get("created_on").map_err(MappingError::db)?,
            content: value.try_get("content").map_err(MappingError::db)?,
        })
    }
}
