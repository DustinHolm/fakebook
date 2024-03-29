use std::{fs, sync::Arc};

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextValidation},
    SchemaBuilder, ServerError, ValidationResult,
};
use axum::async_trait;
use deadpool_postgres::Pool;
use tracing::debug;

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
        .extension(ComplexityExtensionFactory)
        .limit_complexity(1000)
        .finish()
}

pub fn save_schema(path: &str) -> Result<(), InfrastructureError> {
    let schema = schema_builder().finish();
    fs::write(path, schema.sdl())?;
    Ok(())
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;

struct ComplexityExtensionFactory;

impl ExtensionFactory for ComplexityExtensionFactory {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ComplexityExtension)
    }
}

struct ComplexityExtension;

#[async_trait]
impl Extension for ComplexityExtension {
    async fn validation(
        &self,
        ctx: &ExtensionContext<'_>,
        next: NextValidation<'_>,
    ) -> Result<ValidationResult, Vec<ServerError>> {
        let res = next.run(ctx).await;
        debug!("complexity: {:?}", res.as_ref().map(|it| it.complexity));
        res
    }
}
