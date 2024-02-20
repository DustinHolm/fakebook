use async_graphql::NewType;
use postgres_types::{FromSql, ToSql};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, ToSql, FromSql, NewType)]
#[postgres(transparent)]
pub struct DbId(i32);

impl Deref for DbId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait HasDbId {
    fn db_id(&self) -> DbId;
}
