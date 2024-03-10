use async_graphql_axum::GraphQLSubscription;
use axum::{routing::get, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, cors::CorsLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};

use super::{app_state::AppState, handlers};

pub fn new(app_state: AppState) -> Router {
    // Wrapped top to bottom
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .into_inner();

    // Wrapped bottom to top
    Router::new()
        .route("/health-check", get(handlers::health_check))
        .route(
            "/graphql",
            get(handlers::graphiql).post(handlers::graphql_handler),
        )
        .route_service(
            "/graphql/ws",
            GraphQLSubscription::new(app_state.schema.clone()),
        )
        .layer(middleware)
        .with_state(app_state)
}
