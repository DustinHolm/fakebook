use thiserror::Error;

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
    InternalData(String),
    #[error("Invalid internal state: {0}")]
    InvalidState(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Other server returned error: {0}")]
    OtherServer(String),
}

impl From<reqwest::Error> for GqlError {
    fn from(value: reqwest::Error) -> Self {
        Self::OtherServer(value.to_string())
    }
}

impl From<async_graphql::Error> for GqlError {
    fn from(value: async_graphql::Error) -> Self {
        Self::InternalData(value.message)
    }
}
