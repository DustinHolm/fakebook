use std::fs;

use async_graphql::{EmptySubscription, SchemaBuilder};

use crate::{
    domain::schema::{RootMutation, RootQuery},
    errors::fatal::FatalError,
};

use super::db::Saver;

fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(RootQuery, RootMutation, EmptySubscription)
}

pub fn new(saver: Saver) -> Schema {
    schema_builder().data(saver).finish()
}

pub fn save_schema(path: &str) -> Result<(), FatalError> {
    let schema = schema_builder().finish();
    fs::write(path, schema.sdl())?;
    Ok(())
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;
