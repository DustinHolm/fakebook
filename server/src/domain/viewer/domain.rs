use crate::domain::app_user::AppUser;

pub struct Viewer {
    pub(super) user: AppUser,
}

impl Viewer {
    pub fn new(user: AppUser) -> Self {
        Self { user }
    }
}
