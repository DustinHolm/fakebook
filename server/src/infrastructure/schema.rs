use std::{fs, sync::Arc};

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextValidation},
    SchemaBuilder, ServerError, ValidationResult,
};
use async_trait::async_trait;
use tracing::debug;

use crate::domain::schema::{RootMutation, RootQuery, RootSubscription};

use super::{
    db::{Loaders, Repo},
    errors::InfrastructureError,
    notification_center::NotificationCenter,
};

fn schema_builder() -> SchemaBuilder<RootQuery, RootMutation, RootSubscription> {
    Schema::build(RootQuery, RootMutation, RootSubscription)
}

pub fn new(repo: Repo, notification_center: NotificationCenter) -> Schema {
    schema_builder()
        // Will get overriden for every request. This is a fallback for subscriptions.
        .data(Loaders::new(repo.clone()))
        .data(notification_center)
        .data(repo)
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

        debug!(
            "Complexity: {:?}",
            res.as_ref()
                .map(|it| it.complexity.to_string())
                .unwrap_or("unknown".to_string())
        );

        res
    }
}
