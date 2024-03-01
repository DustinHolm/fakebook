use deadpool_postgres::PoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Could not connect to db: {0}")]
    ConnectionFailed(#[source] PoolError),
    #[error("Could not parse row: {0}")]
    Mapping(#[source] tokio_postgres::Error),
    #[error("Could not execute statement: {0}")]
    Statement(#[source] tokio_postgres::Error),
}

impl DbError {
    pub fn mapping(e: tokio_postgres::Error) -> Self {
        Self::Mapping(e)
    }

    pub fn statement(e: tokio_postgres::Error) -> Self {
        Self::Statement(e)
    }
}

impl From<PoolError> for DbError {
    fn from(e: PoolError) -> Self {
        Self::ConnectionFailed(e)
    }
}

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Could not decode relay id: {0}")]
    DecodeRelayId(String),
}

impl MappingError {
    pub fn decode_relay_id(e: impl std::error::Error) -> Self {
        Self::DecodeRelayId(e.to_string())
    }
}

#[derive(Debug, Error)]
pub enum GqlError {
    #[error("Could not load from db")]
    DbLoad,
    #[error("Could not save to db")]
    DbSave,
    #[error("Could not access internal tooling")]
    InternalData,
    #[error("Invalid internal state: {0}")]
    InvalidState(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
