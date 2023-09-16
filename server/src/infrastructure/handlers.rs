use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use deadpool_postgres::Pool;

use crate::errors::fatal::FatalError;

use super::schema::Schema;

pub async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn health_check(pool: Extension<Pool>) -> Result<(), FatalError> {
    let _ = pool.get().await?;

    Ok(())
}
