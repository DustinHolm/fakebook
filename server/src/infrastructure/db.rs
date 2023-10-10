use std::ops::DerefMut;

use async_graphql::dataloader::{DataLoader, HashMapCache};
use deadpool_postgres::{Config, Pool, Runtime};
use refinery::embed_migrations;
use tokio::spawn;
use tokio_postgres::NoTls;
use tracing::instrument;

use crate::{
    errors::fatal::FatalError,
    models::{
        app_user::{AppUserLoader, FriendIdLoader},
        comment::CommentsOfPostLoader,
        post::{PostLoader, PostsOfAuthorLoader},
    },
};

embed_migrations!();

#[instrument(skip_all, err(Debug))]
pub fn create_pool() -> Result<Pool, FatalError> {
    let mut config = Config::new();
    let port = dotenv::var("PG_PORT")?;
    config.host = Some(dotenv::var("PG_HOST")?);
    config.port = Some(port.parse()?);
    config.dbname = Some(dotenv::var("PG_DBNAME")?);
    config.user = Some(dotenv::var("PG_USER")?);
    config.password = Some(dotenv::var("PG_PASSWORD")?);

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;

    Ok(pool)
}

#[instrument(skip_all, err(Debug))]
pub async fn migrate(pool: &Pool) -> Result<(), FatalError> {
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
            comments_of_post: DataLoader::with_cache(
                CommentsOfPostLoader::new(pool),
                spawn,
                HashMapCache::default(),
            ),
        }
    }
}
