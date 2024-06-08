use deadpool_postgres::Pool;

use super::{
    db::Repo,
    schema::{self, Schema},
};

#[derive(Clone)]
pub struct AppState {
    pub(super) repo: Repo,
    pub(super) schema: Schema,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        let repo = Repo::new(pool);
        let schema = schema::new(repo.clone());

        Self { repo, schema }
    }
}
