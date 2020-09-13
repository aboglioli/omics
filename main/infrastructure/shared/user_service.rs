use std::sync::Arc;

use async_trait::async_trait;

use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::user::UserRepository;
use shared::domain::user::{User, UserId, UserService};

pub struct LocalUserService {
    user_repo: Arc<dyn UserRepository>,
}

impl LocalUserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        LocalUserService { user_repo }
    }
}

#[async_trait]
impl UserService for LocalUserService {
    async fn get_by_id(&self, id: &UserId) -> Result<User> {
        let user = self.user_repo.find_by_id(id).await?;

        Ok(User::build(
            AggregateRoot::new(id.clone()),
            user.identity().username().to_string(),
            user.person()
                .map(|person| person.fullname().name().to_string()),
            user.person()
                .map(|person| person.fullname().lastname().to_string()),
            user.person()
                .map(|p| p.biography().map(|b| b.to_string()))
                .flatten(),
            user.person()
                .map(|p| p.profile_image().map(|i| i.to_string()))
                .flatten(),
            user.role_id().to_string(),
        ))
    }
}
