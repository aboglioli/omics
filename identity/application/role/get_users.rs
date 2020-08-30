use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::UserDto;
use crate::domain::role::RoleId;
use crate::domain::user::{UserId, UserRepository};

#[derive(Serialize)]
pub struct GetUsersResponse {
    pub users: Vec<UserDto>,
}

pub struct GetUsers<'a> {
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetUsers<'a> {
    pub fn new(user_repo: &'a dyn UserRepository) -> Self {
        GetUsers { user_repo }
    }

    pub async fn exec(&self, auth_id: String, role_id: String) -> Result<GetUsersResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        let users = self
            .user_repo
            .find_by_role_id(&RoleId::new(role_id)?)
            .await?;

        Ok(GetUsersResponse {
            users: users.iter().map(UserDto::from).collect(),
        })
    }
}
