use async_trait::async_trait;

use common::result::Result;

use crate::domain::user::{User, UserId, UserService};

pub struct FakeUserService;

impl FakeUserService {
    pub fn new() -> Self {
        FakeUserService
    }
}

#[async_trait]
impl UserService for FakeUserService {
    async fn get_by_id(&self, _id: &UserId) -> Result<User> {
        User::new(UserId::new("user-1")?, "user-1", "user")
    }
}
