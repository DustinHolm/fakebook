use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use deadpool_postgres::Pool;
use tracing::instrument;

use crate::errors::fatal::FatalError;

use super::{db::Loaders, schema::Schema};

pub async fn graphql_handler(
    schema: Extension<Schema>,
    pool: Extension<Pool>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req_with_loaders = req.into_inner().data(Loaders::new(pool.0));

    schema.execute(req_with_loaders).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[instrument(skip_all, err(Debug))]
pub async fn health_check(pool: Extension<Pool>, _: Extension<Schema>) -> Result<(), FatalError> {
    let _ = pool.get().await?;

    Ok(())
}
