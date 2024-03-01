use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use axum::async_trait;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::instrument;

use crate::{
    domain::{db_id::DbId, errors::DbError},
    infrastructure::db::Saver,
};

use super::Comment;

pub struct CommentLoader {
    pool: Pool,
}

impl CommentLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Loader<DbId> for CommentLoader {
    type Value = Comment;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;

        let stmt = db
            .prepare_cached("SELECT * FROM comment WHERE comment_id = ANY($1)")
            .await
            .map_err(DbError::statement)?;

        let rows = db.query(&stmt, &[&ids]).await.map_err(DbError::statement)?;

        rows.into_iter()
            .map(|row| {
                let comment: Comment = row.try_into()?;
                Ok((comment.comment_id, comment))
            })
            .collect()
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
impl Loader<DbId> for CommentsOfPostLoader {
    type Value = Vec<Comment>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;

        let stmt = db
            .prepare_cached("SELECT * FROM comment WHERE referenced_post = ANY($1)")
            .await
            .map_err(DbError::statement)?;

        let rows = db.query(&stmt, &[&ids]).await.map_err(DbError::statement)?;

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

impl Saver {
    #[instrument(skip(self), err)]
    pub async fn save_comment(
        &self,
        author_id: &DbId,
        referenced_post_id: &DbId,
        content: &str,
    ) -> Result<Comment, DbError> {
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
            .await
            .map_err(DbError::statement)?;

        let row = db
            .query_one(&stmt, &[&author_id, &now, &content, &referenced_post_id])
            .await
            .map_err(DbError::statement)?;

        row.try_into()
    }
}

impl TryFrom<Row> for Comment {
    type Error = DbError;

    #[instrument(err)]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Comment {
            comment_id: value.try_get("comment_id").map_err(DbError::mapping)?,
            referenced_post: value.try_get("referenced_post").map_err(DbError::mapping)?,
            author: value.try_get("author").map_err(DbError::mapping)?,
            created_on: value.try_get("created_on").map_err(DbError::mapping)?,
            content: value.try_get("content").map_err(DbError::mapping)?,
        })
    }
}
