use async_graphql::{EmptyMutation, EmptySubscription};

use crate::models::schema::RootQuery;

pub fn new() -> Schema {
    Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish()
}

pub type Schema = async_graphql::Schema<RootQuery, EmptyMutation, EmptySubscription>;
