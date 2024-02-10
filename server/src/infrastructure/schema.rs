use std::fs;

use async_graphql::EmptySubscription;

use crate::{
    errors::fatal::FatalError,
    models::schema::{RootMutation, RootQuery},
};

use super::db::Saver;

pub fn new(saver: Saver) -> Schema {
    Schema::build(RootQuery, RootMutation, EmptySubscription)
        .data(saver)
        .finish()
}

pub fn save_schema(path: &str) -> Result<(), FatalError> {
    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription).finish();
    fs::write(path, schema.sdl())?;
    Ok(())
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;
