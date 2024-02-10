use axum::{routing::get, Extension, Router};
use deadpool_postgres::Pool;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, cors::CorsLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};

use super::handlers;
use super::schema::Schema;

pub fn new(pool: Pool, schema: Schema) -> Router {
    // Wrapped top to bottom
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(Extension(schema))
        .layer(Extension(pool))
        .into_inner();

    // Wrapped bottom to top
    Router::new()
        .route("/health-check", get(handlers::health_check))
        .route(
            "/graphql",
            get(handlers::graphiql).post(handlers::graphql_handler),
        )
        .layer(middleware)
}
