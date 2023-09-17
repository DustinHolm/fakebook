use async_graphql::dataloader::{DataLoader, HashMapCache};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio::spawn;
use tokio_postgres::NoTls;

use crate::{
    errors::fatal::FatalError,
    models::{
        app_user::{AppUserLoader, FriendIdLoader},
        post::PostLoader,
    },
};

pub fn create_pool() -> Result<Pool, FatalError> {
    let mut config = Config::new();
    let port = dotenv::var("PG_PORT")?;
    config.host = Some(dotenv::var("PG_HOST")?);
    config.port = Some(u16::from_str_radix(&port, 10)?);
    config.dbname = Some(dotenv::var("PG_DBNAME")?);
    config.user = Some(dotenv::var("PG_USER")?);
    config.password = Some(dotenv::var("PG_PASSWORD")?);

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;

    Ok(pool)
}

pub struct Loaders {
    pub app_user: DataLoader<AppUserLoader, HashMapCache>,
    pub friend_id: DataLoader<FriendIdLoader, HashMapCache>,
    pub post: DataLoader<PostLoader, HashMapCache>,
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
        }
    }
}
