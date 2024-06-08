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
    InternalData,
    #[error("Invalid internal state: {0}")]
    InvalidState(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
