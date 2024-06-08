use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::{instrument, Level};

use crate::{
    domain::db_id::DbId,
    infrastructure::{db::Repo, DbError},
};

use super::domain::Post;

pub struct PostLoader {
    repo: Repo,
}

impl PostLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for PostLoader {
    type Value = Post;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        self.repo
            .query(
                "SELECT * FROM post WHERE post_id = ANY($1)",
                &[&ids],
                |rows| {
                    rows.into_iter()
                        .map(|row| {
                            let post: Post = row.try_into()?;
                            Ok::<_, DbError>((post.post_id, post))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                },
            )
            .await
            .map_err(|e| e.into())
    }
}

pub struct PostsOfAuthorLoader {
    repo: Repo,
}

impl PostsOfAuthorLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for PostsOfAuthorLoader {
    type Value = Vec<Post>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let posts: Vec<Post> = self
            .repo
            .query(
                "SELECT * FROM post WHERE author = ANY($1)",
                &[&ids],
                |rows| rows.into_iter().map(|row| row.try_into()).collect(),
            )
            .await?;

        let mut result = HashMap::from_iter(ids.iter().map(|id| (*id, Vec::new())));

        for post in posts {
            result
                .entry(post.author)
                .and_modify(|e: &mut Vec<Post>| e.push(post));
        }

        Ok(result)
    }
}

impl Repo {
    #[instrument(skip(self), err)]
    pub async fn save_post(&self, author_id: &DbId, content: &str) -> Result<Post, DbError> {
        let now = OffsetDateTime::now_utc();

        self.query_one(
            r"
                INSERT INTO post (author, created_on, content)
                VALUES ($1, $2, $3)
                RETURNING *
            ",
            &[author_id, &now, &content],
            |row| row.try_into(),
        )
        .await
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
