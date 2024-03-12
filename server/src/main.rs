mod domain;
mod infrastructure;

use std::net::SocketAddr;

use axum::serve;
use tokio::net::TcpListener;

use crate::infrastructure::{app_state::AppState, db, logging, router, schema, shutdown};

#[tokio::main]
async fn main() {
    schema::save_schema("./schema.graphql").expect("Should have written schema to file");
    dotenv::dotenv().expect(".env file should be available");

    let _guard = logging::init().expect("Logging should build"); // Guard flushes when main/server stops

    let pool = db::create_pool().expect("Pool should have been created");
    db::migrate(&pool).await.expect("Migrations should succeed");

    let app_state = AppState::new(pool);
    let router = router::new(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Should have bound to port");

    tracing::info!("Listening on {}", addr);
    tracing::info!("Visit GraphiQL: http://{}/graphql", addr);

    serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .expect("Server should start");
}
