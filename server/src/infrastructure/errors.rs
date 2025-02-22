use axum::response::{IntoResponse, Response};
use deadpool_postgres::{BuildError, PoolError};
use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Db failed on pool connection: {0}")]
    DbPoolConnection(#[from] PoolError),
    #[error("Db failed on separate connection: {0}")]
    DbExplicitConnection(#[from] tokio_postgres::Error),
    #[error("Db failed on startup: {0}")]
    DbStartup(#[from] BuildError),
    #[error("Env had missing values: {0}")]
    Env(#[from] dotenvy::Error),
    #[error("Env had invalid values: {0}")]
    EnvInvalid(String),
    #[error("Filesystem did not cooperate: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("Health check failed: {0}")]
    HealthCheck(#[source] Box<dyn std::error::Error>),
    #[error("Logging could not start: {0}")]
    Logging(#[source] Box<dyn std::error::Error>),
    #[error("Migrations failed: {0}")]
    Migrations(#[source] Box<refinery::Error>),
}

impl From<refinery::Error> for InfrastructureError {
    fn from(value: refinery::Error) -> Self {
        Self::Migrations(Box::new(value))
    }
}

impl InfrastructureError {
    pub fn env_invalid(msg: String) -> Self {
        Self::EnvInvalid(msg)
    }

    pub fn health(e: impl std::error::Error + 'static) -> Self {
        Self::HealthCheck(Box::new(e))
    }
}

impl IntoResponse for InfrastructureError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

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
pub enum NotificationCenterError {
    #[error("The daemon failed to start: {0}")]
    DaemonFailedToStart(String),
    #[error("The daemon seems to be dead: {0}")]
    DaemonDead(String),
    #[error("Failed to subscribe to a topic")]
    SubscriptionFailed,
    #[error("Failed to parse Notification")]
    ParsingFailed,
}
