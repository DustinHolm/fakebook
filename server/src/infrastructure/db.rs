use std::{future::Future, ops::DerefMut};

use async_graphql::dataloader::{DataLoader, HashMapCache};
use deadpool_postgres::{Config, Pool, Runtime};
use postgres_types::ToSql;
use tokio::{spawn, task::JoinHandle};
use tokio_postgres::{NoTls, Row};
use tracing::{instrument, Instrument};

use crate::domain::{
    app_user::{AppUserLoader, FriendIdLoader},
    comment::{CommentLoader, CommentsOfPostLoader},
    post::{PostLoader, PostsOfAuthorLoader},
};

use super::errors::{DbError, InfrastructureError};

mod default {
    use refinery::embed_migrations;

    embed_migrations!();
}

mod debug {
    use refinery::embed_migrations;

    embed_migrations!("migrations/debug");
}

#[instrument(skip_all, err)]
pub fn create_pool() -> Result<Pool, InfrastructureError> {
    let mut config = Config::new();
    let port = dotenv::var("PG_PORT")?;
    config.host = Some(dotenv::var("PG_HOST")?);
    config.port = Some(
        port.parse()
            .map_err(|_| InfrastructureError::env_invalid(format!("Invalid port: {port}")))?,
    );
    config.dbname = Some(dotenv::var("PG_DBNAME")?);
    config.user = Some(dotenv::var("PG_USER")?);
    config.password = Some(dotenv::var("PG_PASSWORD")?);

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;

    Ok(pool)
}

#[instrument(skip_all, err)]
pub async fn migrate(pool: &Pool) -> Result<(), InfrastructureError> {
    let mut db = pool.get().await?;

    default::migrations::runner()
        .set_abort_missing(false)
        .run_async(db.deref_mut().deref_mut())
        .await?;

    debug::migrations::runner()
        .set_abort_missing(false)
        .run_async(db.deref_mut().deref_mut())
        .await?;

    Ok(())
}

fn spawn_in_span<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    spawn(future.in_current_span())
}

#[derive(Clone)]
pub struct Repo {
    pool: Pool,
}

impl Repo {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self), err)]
    pub async fn query_one<T, Err: Into<DbError>>(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
        mapper: fn(Row) -> Result<T, Err>,
    ) -> Result<T, DbError> {
        let db = self.pool.get().await?;

        let prepared_statement = db
            .prepare_cached(statement)
            .await
            .map_err(DbError::statement)?;

        let row = db
            .query_one(&prepared_statement, params)
            .await
            .map_err(DbError::statement)?;

        mapper(row).map_err(|e| e.into())
    }

    #[instrument(skip(self), err)]
    pub async fn query<T, Err: Into<DbError>>(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
        mapper: fn(Vec<Row>) -> Result<T, Err>,
    ) -> Result<T, DbError> {
        let db = self.pool.get().await?;

        let prepared_statement = db
            .prepare_cached(statement)
            .await
            .map_err(DbError::statement)?;

        let rows = db
            .query(&prepared_statement, params)
            .await
            .map_err(DbError::statement)?;

        mapper(rows).map_err(|e| e.into())
    }

    #[instrument(skip(self), err)]
    pub async fn execute(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<(), DbError> {
        let db = self.pool.get().await?;

        let prepared_statement = db
            .prepare_cached(statement)
            .await
            .map_err(DbError::statement)?;

        db.execute(&prepared_statement, params)
            .await
            .map_err(DbError::statement)?;

        Ok(())
    }

    #[instrument(skip(self), err)]
    pub(super) async fn health(&self) -> Result<(), DbError> {
        self.pool.get().await.map(|_| ()).map_err(|e| e.into())
    }
}

pub struct Loaders {
    pub app_user: DataLoader<AppUserLoader, HashMapCache>,
    pub friend_id: DataLoader<FriendIdLoader, HashMapCache>,
    pub post: DataLoader<PostLoader, HashMapCache>,
    pub posts_of_author: DataLoader<PostsOfAuthorLoader, HashMapCache>,
    pub comment: DataLoader<CommentLoader, HashMapCache>,
    pub comments_of_post: DataLoader<CommentsOfPostLoader, HashMapCache>,
}

impl Loaders {
    pub fn new(repo: Repo) -> Self {
        Self {
            app_user: DataLoader::with_cache(
                AppUserLoader::new(repo.clone()),
                spawn_in_span,
                HashMapCache::default(),
            ),
            friend_id: DataLoader::with_cache(
                FriendIdLoader::new(repo.clone()),
                spawn_in_span,
                HashMapCache::default(),
            ),
            post: DataLoader::with_cache(
                PostLoader::new(repo.clone()),
                spawn_in_span,
                HashMapCache::default(),
            ),
            posts_of_author: DataLoader::with_cache(
                PostsOfAuthorLoader::new(repo.clone()),
                spawn_in_span,
                HashMapCache::default(),
            ),
            comment: DataLoader::with_cache(
                CommentLoader::new(repo.clone()),
                spawn_in_span,
                HashMapCache::default(),
            ),
            comments_of_post: DataLoader::with_cache(
                CommentsOfPostLoader::new(repo),
                spawn_in_span,
                HashMapCache::default(),
            ),
        }
    }

    pub fn clear_caches(&self) {
        self.app_user.clear();
        self.friend_id.clear();
        self.post.clear();
        self.posts_of_author.clear();
        self.comment.clear();
        self.comments_of_post.clear();
    }
}
