use super::{
    db::Repo,
    notification_center::NotificationCenter,
    schema::{self, Schema},
    urls::Urls,
};

#[derive(Clone)]
pub struct AppState {
    pub(super) repo: Repo,
    pub(super) schema: Schema,
}

impl AppState {
    pub fn new(notification_center: NotificationCenter, repo: Repo, urls: Urls) -> Self {
        let schema = schema::new(repo.clone(), notification_center, urls);

        Self { repo, schema }
    }
}
