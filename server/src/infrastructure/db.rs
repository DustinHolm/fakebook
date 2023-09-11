use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::errors::fatal::FatalError;

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
