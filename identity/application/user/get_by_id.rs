use std::sync::Arc;

use common::error::Error;

use crate::domain::user::{User, UserId, UserRepository};

pub struct GetById {
    user_repo: Arc<dyn UserRepository>,
}

impl GetById {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        GetById { user_repo }
    }

    pub fn exec(&self, user_id: &UserId) -> Result<User, Error> {
        let user = self.user_repo.find_by_id(user_id)?;
        Ok(user)
    }
}
