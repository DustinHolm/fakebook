use async_graphql::ID;
use time::OffsetDateTime;

use crate::domain::{
    db_id::{CanDecodeId, DbId, HasDbId},
    errors::MappingError,
};

pub const SUFFIX: &str = "Comment";

#[derive(Clone)]
pub struct Comment {
    pub comment_id: DbId,
    pub(super) referenced_post: DbId,
    pub(super) author: DbId,
    pub(super) created_on: OffsetDateTime,
    pub(super) content: String,
}

impl HasDbId for Comment {
    fn db_id(&self) -> DbId {
        self.comment_id
    }
}

impl CanDecodeId for Comment {
    fn decode(relay_id: &ID) -> Result<DbId, MappingError> {
        Self::decode_with_suffix(relay_id, SUFFIX)
    }
}
