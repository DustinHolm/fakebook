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

use super::{app_user::AppUser, post::Post};

#[derive(Clone)]
pub struct Comment {
    comment_id: i32,
    referenced_post: i32,
    author: i32,
    created_on: OffsetDateTime,
    content: String,
}

#[Object]
impl Comment {
    /// Id used by relay. Must be globally unique.
    async fn id(&self) -> ID {
        let combined = self.comment_id.to_string() + "Comment";

        ID(STANDARD.encode(combined))
    }

    async fn pid(&self) -> ID {
        ID(self.comment_id.to_string())
    }

    #[instrument(skip_all, err(Debug))]
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
}

pub struct CommentsOfPostLoader {
    pool: Pool,
}

impl CommentsOfPostLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<i32> for CommentsOfPostLoader {
    type Value = Vec<Comment>;
    type Error = DbError;

    #[instrument(skip(self), err(Debug))]
    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(DbError::connection)?;

        let stmt = db
            .prepare_cached("SELECT * FROM comment WHERE referenced_post = ANY($1)")
            .await?;

        let rows = db.query(&stmt, &[&ids]).await?;

        let mut result = HashMap::from_iter(ids.iter().map(|id| (*id, Vec::new())));

        for row in rows {
            let comment: Comment = row.try_into()?;
            result
                .entry(comment.referenced_post)
                .and_modify(|e: &mut Vec<Comment>| e.push(comment));
        }

        Ok(result)
    }
}

#[derive(Debug, InputObject)]
pub struct CommentInput {
    author: i32,
    content: String,
    referenced_post: i32,
}

impl Saver {
    #[instrument(skip(self), err(Debug))]
    pub async fn save_comment(&self, comment: &CommentInput) -> Result<Comment, DbError> {
        let db = self.pool.get().await?;

        let now = OffsetDateTime::now_utc();

        let stmt = db
            .prepare_cached(
                r"
                    INSERT INTO comment (author, created_on, content, referenced_post)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                ",
            )
            .await?;

        let row = db
            .query_one(
                &stmt,
                &[
                    &comment.author,
                    &now,
                    &comment.content,
                    &comment.referenced_post,
                ],
            )
            .await?;

        Ok(row.try_into()?)
    }
}

impl TryFrom<Row> for Comment {
    type Error = MappingError;

    #[instrument(err(Debug))]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Comment {
            comment_id: value.try_get("comment_id").map_err(MappingError::db)?,
            referenced_post: value.try_get("referenced_post").map_err(MappingError::db)?,
            author: value.try_get("author").map_err(MappingError::db)?,
            created_on: value.try_get("created_on").map_err(MappingError::db)?,
            content: value.try_get("content").map_err(MappingError::db)?,
        })
    }
}
