use std::fmt::{Debug, Display};

use super::base::Error;

enum Reason {
    Other,
    FromDb,
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::Other => write!(f, "Error while mapping between types"),
            Reason::FromDb => write!(f, "Error while mapping from DB"),
        }
    }
}

pub struct MappingError {
    reason: Reason,
    message: String,
}

impl MappingError {
    pub fn from_db(error: tokio_postgres::Error) -> Self {
        Self {
            reason: Reason::FromDb,
            message: error.to_string(),
        }
    }
}

impl<E: std::error::Error + 'static> Error<E> for MappingError {}

impl<E> From<E> for MappingError
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

impl Debug for MappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.reason, self.message)
    }
}

impl Display for MappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
