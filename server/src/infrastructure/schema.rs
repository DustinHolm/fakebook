use async_graphql::EmptySubscription;

use crate::models::schema::{RootMutation, RootQuery};

use super::db::Saver;

pub fn new(savor: Saver) -> Schema {
    Schema::build(RootQuery, RootMutation, EmptySubscription)
        .data(savor)
        .finish()
}

pub type Schema = async_graphql::Schema<RootQuery, RootMutation, EmptySubscription>;
