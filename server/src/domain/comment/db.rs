use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::{instrument, Level};

use crate::{
    domain::db_id::DbId,
    infrastructure::{db::Repo, DbError},
};

use super::Comment;

pub struct CommentLoader {
    repo: Repo,
}

impl CommentLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for CommentLoader {
    type Value = Comment;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        self.repo
            .query(
                "SELECT * FROM comment WHERE comment_id = ANY($1)",
                &[&ids],
                |rows| {
                    rows.into_iter()
                        .map(|row| {
                            let comment: Comment = row.try_into()?;
                            Ok::<_, DbError>((comment.comment_id, comment))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                },
            )
            .await
            .map_err(|e| e.into())
    }
}

pub struct CommentsOfPostLoader {
    repo: Repo,
}

impl CommentsOfPostLoader {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl Loader<DbId> for CommentsOfPostLoader {
    type Value = Vec<Comment>;
    type Error = Arc<DbError>;

    #[instrument(skip(self), err)]
    async fn load(&self, ids: &[DbId]) -> Result<HashMap<DbId, Self::Value>, Self::Error> {
        let comments: Vec<Comment> = self
            .repo
            .query(
                "SELECT * FROM comment WHERE referenced_post = ANY($1)",
                &[&ids],
                |rows| rows.into_iter().map(|row| row.try_into()).collect(),
            )
            .await?;

        let mut result = HashMap::from_iter(ids.iter().map(|id| (*id, Vec::new())));

        for comment in comments {
            result
                .entry(comment.referenced_post)
                .and_modify(|old| old.push(comment));
        }

        Ok(result)
    }
}

impl Repo {
    #[instrument(skip(self), err)]
    pub async fn save_comment(
        &self,
        author_id: &DbId,
        referenced_post_id: &DbId,
        content: &str,
    ) -> Result<Comment, DbError> {
        let now = OffsetDateTime::now_utc();

        self.query_one(
            r"
                INSERT INTO comment (author, created_on, content, referenced_post)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            ",
            &[&author_id, &now, &content, &referenced_post_id],
            |row| row.try_into(),
        )
        .await
    }
}

impl TryFrom<Row> for Comment {
    type Error = DbError;

    #[instrument(level = Level::TRACE, err)]
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
