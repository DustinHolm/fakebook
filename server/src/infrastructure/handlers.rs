use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use deadpool_postgres::Pool;
use tracing::instrument;

use super::{db::Loaders, errors::InfrastructureError, schema::Schema};

pub async fn graphql_handler(
    Extension(schema): Extension<Schema>,
    Extension(pool): Extension<Pool>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req_with_loaders = req.into_inner().data(Loaders::new(pool));

    schema.execute(req_with_loaders).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/graphql/ws")
            .finish(),
    )
}

#[instrument(skip_all, err)]
pub async fn health_check(
    Extension(pool): Extension<Pool>,
    _: Extension<Schema>,
) -> Result<(), InfrastructureError> {
    let _ = pool.get().await.map_err(InfrastructureError::health)?;

    Ok(())
}
