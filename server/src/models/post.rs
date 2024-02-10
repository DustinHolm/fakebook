use std::collections::HashMap;

use async_graphql::{dataloader::Loader, Context, InputObject, Object, ID};
use axum::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    errors::{db::DbError, mapping::MappingError, query::QueryError},
    infrastructure::db::{Loaders, Saver},
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
    /// Id used by relay. Must be globally unique.
    async fn id(&self) -> ID {
        let combined = self.post_id.to_string() + "Post";

        ID(STANDARD.encode(combined))
    }

    async fn pid(&self) -> ID {
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
    type Error = DbError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(DbError::connection)?;
        let stmt = db
            .prepare_cached("SELECT * FROM post WHERE post_id = ANY($1)")
            .await?;

        let rows = db.query(&stmt, &[&ids]).await?;

        rows.into_iter()
            .map(|row| {
                let post: Post = row.try_into()?;
                Ok((post.post_id, post))
            })
            .collect()
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
    type Error = DbError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(DbError::connection)?;

        let stmt = db
            .prepare_cached("SELECT * FROM post WHERE author = ANY($1)")
            .await?;

        let rows = db.query(&stmt, &[&ids]).await?;

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

#[derive(Debug, InputObject)]
pub struct PostInput {
    author: i32,
    content: String,
}

impl Saver {
    #[instrument(skip(self), err(Debug))]
    pub async fn save_post(&self, post: &PostInput) -> Result<Post, DbError> {
        let db = self.pool.get().await?;

        let now = OffsetDateTime::now_utc();

        let stmt = db
            .prepare_cached(
                r"
                    INSERT INTO post (author, created_on, content)
                    VALUES ($1, $2, $3)
                    RETURNING *
                ",
            )
            .await?;

        let row = db
            .query_one(&stmt, &[&post.author, &now, &post.content])
            .await?;

        Ok(row.try_into()?)
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
