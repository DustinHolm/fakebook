use std::fs;

use async_graphql::SchemaBuilder;
use deadpool_postgres::Pool;

use crate::domain::schema::{RootMutation, RootQuery, RootSubscription};

use super::{
    db::{Loaders, Saver},
    errors::InfrastructureError,
};

fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, RootSubscription> {
    Schema::build(RootQuery, RootMutation, RootSubscription)
}

pub fn new(saver: Saver, pool: Pool) -> Schema {
    schema_builder()
        // Will get overriden for every request. This is a fallback for subscriptions.
        .data(Loaders::new(pool.clone()))
        .data(saver)
        .data(pool)
        .finish()
}

pub fn save_schema(path: &str) -> Result<(), InfrastructureError> {
    let schema = schema_builder().finish();
    fs::write(path, schema.sdl())?;
    Ok(())
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;
