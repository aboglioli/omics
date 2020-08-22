use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::UserDto;
use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetAllResponse {
    pub users: Vec<UserDto>,
}

pub struct GetAll<'a> {
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(user_repo: &'a dyn UserRepository) -> Self {
        GetAll { user_repo }
    }

    pub async fn exec(&self, auth_id: String) -> Result<GetAllResponse> {
        let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !auth_user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        let users = self.user_repo.find_all().await?;

        Ok(GetAllResponse {
            users: users.iter().map(|user| UserDto::from(user)).collect(),
        })
    }
}
