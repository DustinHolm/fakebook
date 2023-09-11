use async_graphql::{EmptyMutation, EmptySubscription};
use deadpool_postgres::Pool;

use crate::models::schema::{RootQuery, Schema};

pub fn new(db_pool: Pool) -> Schema {
    Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish()
}
