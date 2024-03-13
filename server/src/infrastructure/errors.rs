use axum::response::{IntoResponse, Response};
use deadpool_postgres::{CreatePoolError, PoolError};
use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Db failed on initial connection: {0}")]
    DbConnection(#[from] PoolError),
    #[error("Db failed on startup: {0}")]
    DbStartup(#[from] CreatePoolError),
    #[error("Env had missing values: {0}")]
    Env(#[from] dotenv::Error),
    #[error("Env had invalid values: {0}")]
    EnvInvalid(String),
    #[error("Filesystem did not cooperate: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("Health check failed: {0}")]
    HealthCheck(#[source] Box<dyn std::error::Error>),
    #[cfg(otel)]
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
