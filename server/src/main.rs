mod errors;
mod infrastructure;
mod models;

use std::net::SocketAddr;

use axum::Server;

use crate::infrastructure::{db, logging, router, schema, shutdown};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env file should be available");

    logging::init();

    let pool = db::create_pool().expect("Pool should have been created");
    let schema = schema::build_with_dataloaders(pool.clone());
    let router = router::new(pool, schema);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown::signal())
        .await
        .expect("Server should start");

    ()
}
