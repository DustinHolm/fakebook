use std::fmt::{Debug, Display};

use super::{base::Error, db::DbError, mapping::MappingError};

enum Reason {
    Other,
    NotFound,
    Db,
    InvalidInput,
    InternalSystem,
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::Other => write!(f, "Unexpected error"),
            Reason::NotFound => write!(f, "Could not find the requested data"),
            Reason::Db => write!(f, "Encountered an error while accessing the database"),
            Reason::InvalidInput => write!(f, "Given input had the wrong format"),
            Reason::InternalSystem => write!(f, "Internal error"),
        }
    }
}

pub struct QueryError {
    reason: Reason,
    message: Option<String>,
}

impl QueryError {
    pub fn not_found() -> Self {
        Self {
            reason: Reason::NotFound,
            message: None,
        }
    }

    pub fn internal(value: impl Display) -> Self {
        Self {
            reason: Reason::InternalSystem,
            message: Some(value.to_string()),
        }
    }

    pub fn invalid_input(message: String) -> Self {
        Self {
            reason: Reason::InvalidInput,
            message: Some(message),
        }
    }
}

impl From<MappingError> for QueryError {
    fn from(value: MappingError) -> Self {
        Self {
            reason: Reason::Other,
            message: Some(value.to_string()),
        }
    }
}

impl From<DbError> for QueryError {
    fn from(value: DbError) -> Self {
        Self {
            reason: Reason::Db,
            message: Some(value.to_string()),
        }
    }
}

impl<E: std::error::Error + 'static> Error<E> for QueryError {}

impl<E> From<E> for QueryError
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        Self {
            reason: Reason::Other,
            message: Some(value.to_string()),
        }
    }
}

impl Debug for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}: {}", self.reason, message)
        } else {
            Display::fmt(&self, f)
        }
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
