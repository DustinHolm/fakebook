use crate::errors::query::QueryError;

pub mod app_user;
pub mod comment;
mod db_id;
pub mod post;
mod relay_meta;
pub mod schema;

pub trait ValidInput {
    fn validate(&self) -> Result<(), QueryError>;
}
