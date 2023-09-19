use std::fmt::{Debug, Display};

use axum::response::IntoResponse;
use hyper::StatusCode;

use super::base::Error;

pub struct FatalError {
    message: String,
}

impl<E: std::error::Error + 'static> Error<E> for FatalError {}

impl<E> From<E> for FatalError
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl Debug for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self, self.message)
    }
}

impl Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fatal problem")
    }
}

impl IntoResponse for FatalError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
