use async_graphql::ID;
use time::OffsetDateTime;

use crate::domain::{
    db_id::{CanDecodeId, DbId, HasDbId},
    errors::MappingError,
};

pub const SUFFIX: &str = "Post";

#[derive(Clone)]
pub struct Post {
    pub post_id: DbId,
    pub(super) author: DbId,
    pub(super) created_on: OffsetDateTime,
    pub(super) content: String,
}

impl HasDbId for Post {
    fn db_id(&self) -> DbId {
        self.post_id
    }
}

impl CanDecodeId for Post {
    fn decode(relay_id: &ID) -> Result<DbId, MappingError> {
        Self::decode_with_suffix(relay_id, SUFFIX)
    }
}
