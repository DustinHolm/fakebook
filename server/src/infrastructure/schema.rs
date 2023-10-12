use async_graphql::EmptySubscription;
use deadpool_postgres::Pool;

use crate::models::schema::{RootMutation, RootQuery};

pub fn new(pool: Pool) -> Schema {
    Schema::build(RootQuery, RootMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;
