use async_graphql::ID;

use crate::domain::{
    db_id::{CanDecodeId, DbId, HasDbId},
    errors::MappingError,
};

pub const SUFFIX: &str = "AppUser";

#[derive(Clone)]
pub struct AppUser {
    pub(super) user_id: DbId,
    pub(super) first_name: String,
    pub(super) last_name: String,
}

impl HasDbId for AppUser {
    fn db_id(&self) -> DbId {
        self.user_id
    }
}

impl CanDecodeId for AppUser {
    fn decode(relay_id: &ID) -> Result<DbId, MappingError> {
        Self::decode_with_suffix(relay_id, SUFFIX)
    }
}
