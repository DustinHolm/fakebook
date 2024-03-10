use deadpool_postgres::Pool;

use super::{
    db::Saver,
    schema::{self, Schema},
};

#[derive(Clone)]
pub struct AppState {
    pub(super) pool: Pool,
    pub(super) schema: Schema,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        let schema = schema::new(Saver::new(pool.clone()), pool.clone());

        Self { pool, schema }
    }
}
