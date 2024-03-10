use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use tracing::instrument;

use super::{app_state::AppState, db::Loaders, errors::InfrastructureError};

pub async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req_with_loaders = req.into_inner().data(Loaders::new(state.pool));

    state.schema.execute(req_with_loaders).await.into()
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
pub async fn health_check(State(state): State<AppState>) -> Result<(), InfrastructureError> {
    let _ = state
        .pool
        .get()
        .await
        .map_err(InfrastructureError::health)?;

    Ok(())
}
