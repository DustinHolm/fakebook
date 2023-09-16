use std::fmt::{Debug, Display};

use deadpool_postgres::PoolError;

use super::{base::Error, mapping::MappingError};

#[derive(Clone)]
enum Reason {
    Other,
    Mapping,
    ConnectionFailed,
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::Other => write!(f, "Error while using DB"),
            Reason::Mapping => write!(f, "Could not map data as expected"),
            Reason::ConnectionFailed => write!(f, "Error while connecting to DB"),
        }
    }
}

#[derive(Clone)]
pub struct LoaderError {
    reason: Reason,
    message: String,
}

impl LoaderError {
    pub fn connection(value: PoolError) -> Self {
        Self {
            reason: Reason::ConnectionFailed,
            message: value.to_string(),
        }
    }
}

impl From<MappingError> for LoaderError {
    fn from(value: MappingError) -> Self {
        Self {
            reason: Reason::Mapping,
            message: value.to_string(),
        }
    }
}

impl<E: std::error::Error + 'static> Error<E> for LoaderError {}

impl<E> From<E> for LoaderError
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        Self {
            reason: Reason::Other,
            message: value.to_string(),
        }
    }
}

impl Debug for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.reason, self.message)
    }
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
