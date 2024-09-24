use async_graphql::{NewType, ID};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use postgres_types::{FromSql, ToSql};
use std::{
    ops::Deref,
    str::{from_utf8, FromStr},
};

use super::errors::MappingError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, ToSql, FromSql, NewType)]
#[postgres(transparent)]
pub struct DbId(i32);

impl Deref for DbId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DbId {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i32::from_str(s).map(DbId::from)
    }
}

pub trait HasDbId {
    fn db_id(&self) -> DbId;
}

pub trait CanDecodeId {
    fn decode(relay_id: &ID) -> Result<DbId, MappingError>;

    fn decode_with_suffix(ID(relay_id): &ID, suffix: &str) -> Result<DbId, MappingError> {
        let unencoded = URL_SAFE
            .decode(relay_id)
            .map_err(MappingError::decode_relay_id)?;
        let as_string = from_utf8(&unencoded).map_err(MappingError::decode_relay_id)?;
        let trimmed = as_string.trim_end_matches(suffix);
        let parsed: i32 = trimmed.parse().map_err(MappingError::decode_relay_id)?;

        Ok(DbId::from(parsed))
    }
}
