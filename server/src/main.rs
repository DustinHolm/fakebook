mod domain;
mod infrastructure;

use axum::serve;
use infrastructure::notification_center::NotificationCenter;
use tokio::net::TcpListener;

use crate::infrastructure::{app_state::AppState, db, logging, router, schema, shutdown};

#[tokio::main]
async fn main() {
    schema::save_schema("./schema.graphql").expect("Should have written schema to file");
    let _ = dotenvy::dotenv(); // If .env is not found, ENV might be configured already
    let addr = dotenvy::var("HOSTING_ADDRESS").expect("Need to know where to bind app");

    let _guard = logging::init().expect("Logging should build"); // Guard flushes when main/server stops

    let repo = db::initiate_repo()
        .await
        .expect("Repo should have been created");
    db::migrate(&repo).await.expect("Migrations should succeed");

    let mut notification_center = NotificationCenter::new(repo.clone());
    notification_center
        .start_daemon()
        .await
        .expect("NotificationCenter should have started");

    let app_state = AppState::new(notification_center, repo);
    let router = router::new(app_state);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Should have bound to port");

    tracing::info!("Listening on {}", &addr);
    tracing::info!("Visit GraphiQL: http://{}/graphql", &addr);

    serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .expect("Server should start");
}
