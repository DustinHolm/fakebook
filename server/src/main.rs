mod domain;
mod infrastructure;

use std::net::SocketAddr;

use axum::Server;

use crate::infrastructure::{db, logging, router, schema, shutdown};

#[tokio::main]
async fn main() {
    schema::save_schema("./schema.graphql").expect("Should have written schema to file");
    dotenv::dotenv().expect(".env file should be available");

    let _guard = logging::init(); // Guard flushes when main/server stops

    let pool = db::create_pool().expect("Pool should have been created");
    db::migrate(&pool).await.expect("Migrations should succeed");
    let schema = schema::new(db::Saver::new(pool.clone()), pool.clone());
    let router = router::new(pool, schema);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    tracing::info!("Visit GraphiQL: http://{}/graphql", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown::signal())
        .await
        .expect("Server should start");
}
