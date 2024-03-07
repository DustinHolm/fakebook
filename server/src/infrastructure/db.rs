use std::ops::DerefMut;

use async_graphql::dataloader::{DataLoader, HashMapCache};
use deadpool_postgres::{Config, Pool, Runtime};
use refinery::embed_migrations;
use tokio::spawn;
use tokio_postgres::NoTls;
use tracing::instrument;

use crate::domain::{
    app_user::{AppUserLoader, FriendIdLoader},
    comment::{CommentLoader, CommentsOfPostLoader},
    post::{PostLoader, PostsOfAuthorLoader},
};

use super::errors::InfrastructureError;

embed_migrations!();

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
    migrations::runner()
        .run_async(db.deref_mut().deref_mut())
        .await?;
    Ok(())
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
    pub fn new(pool: Pool) -> Self {
        Self {
            app_user: DataLoader::with_cache(
                AppUserLoader::new(pool.clone()),
                spawn,
                HashMapCache::default(),
            ),
            friend_id: DataLoader::with_cache(
                FriendIdLoader::new(pool.clone()),
                spawn,
                HashMapCache::default(),
            ),
            post: DataLoader::with_cache(
                PostLoader::new(pool.clone()),
                spawn,
                HashMapCache::default(),
            ),
            posts_of_author: DataLoader::with_cache(
                PostsOfAuthorLoader::new(pool.clone()),
                spawn,
                HashMapCache::default(),
            ),
            comment: DataLoader::with_cache(
                CommentLoader::new(pool.clone()),
                spawn,
                HashMapCache::default(),
            ),
            comments_of_post: DataLoader::with_cache(
                CommentsOfPostLoader::new(pool),
                spawn,
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

pub struct Saver {
    pub pool: Pool,
}

impl Saver {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
