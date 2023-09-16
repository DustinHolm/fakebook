use std::hash::BuildHasher;

use async_graphql::{
    dataloader::{DataLoader, HashMapCache},
    http::GraphiQLSource,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use deadpool_postgres::Pool;
use tokio::spawn;

use crate::{
    errors::fatal::FatalError,
    models::app_user::{AppUserLoader, FriendIdLoader},
};

use super::schema::Schema;

pub async fn graphql_handler(
    schema: Extension<Schema>,
    pool: Extension<Pool>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req_with_loader = req
        .into_inner()
        .data(DataLoader::with_cache(
            AppUserLoader::new(pool.0.clone()),
            spawn,
            HashMapCache::default(),
        ))
        .data(DataLoader::with_cache(
            FriendIdLoader::new(pool.0),
            spawn,
            HashMapCache::default(),
        ));

    schema.execute(req_with_loader).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn health_check(pool: Extension<Pool>) -> Result<(), FatalError> {
    let _ = pool.get().await?;

    Ok(())
}
