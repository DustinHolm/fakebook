use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::{instrument, Level};

use crate::{
    domain::{db_id::DbId, errors::DbError},
    infrastructure::db::Saver,
};

use super::domain::Post;

pub struct PostLoader {
    pool: Pool,
}

impl PostLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl Loader<DbId> for PostLoader {
    type Value = Post;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;
        let stmt = db
            .prepare_cached("SELECT * FROM post WHERE post_id = ANY($1)")
            .await
            .map_err(DbError::statement)?;

        let rows = db.query(&stmt, &[&ids]).await.map_err(DbError::statement)?;

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

impl Loader<DbId> for PostsOfAuthorLoader {
    type Value = Vec<Post>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let db = self.pool.get().await.map_err(|e| Arc::new(e.into()))?;

        let stmt = db
            .prepare_cached("SELECT * FROM post WHERE author = ANY($1)")
            .await
            .map_err(DbError::statement)?;

        let rows = db.query(&stmt, &[&ids]).await.map_err(DbError::statement)?;

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

impl Saver {
    #[instrument(skip(self), err)]
    pub async fn save_post(&self, author_id: &DbId, content: &str) -> Result<Post, DbError> {
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
            .await
            .map_err(DbError::statement)?;

        let row = db
            .query_one(&stmt, &[author_id, &now, &content])
            .await
            .map_err(DbError::statement)?;

        row.try_into()
    }
}

impl TryFrom<Row> for Post {
    type Error = DbError;

    #[instrument(level = Level::TRACE, err)]
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Post {
            post_id: value.try_get("post_id").map_err(DbError::mapping)?,
            author: value.try_get("author").map_err(DbError::mapping)?,
            created_on: value.try_get("created_on").map_err(DbError::mapping)?,
            content: value.try_get("content").map_err(DbError::mapping)?,
        })
    }
}
