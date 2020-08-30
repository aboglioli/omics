use common::error::Error;
use common::result::Result;

use crate::application::dtos::UserDto;
use crate::domain::user::{UserRepository, UserId};
use crate::domain::role::{RoleId};

#[derive(Serialize)]
pub struct SearchResponse {
    users: Vec<UserDto>,
}

pub struct QueryService<'a> {
    user_repo: &'a dyn UserRepository,
}

impl<'a> QueryService<'a> {
    pub fn new(
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        QueryService {
            user_repo,
        }
    }

    pub async fn gey_by_id(&self, auth_id: String, user_id: String) -> Result<UserDto> {
        if auth_id != user_id {
            let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            if !auth_user.role().is("admin") {
                return Err(Error::unauthorized());
            }
        }

        let user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

        Ok(UserDto::from(&user))
    }

    pub async fn get_by_role_id(&self, auth_id: String, role_id: String) -> Result<CollectionResponse> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        let users = self.user_repo.find_by_role_id(&RoleId::new(role_id)?).await?;

        Ok(SearchResponse {
            users: users.iter().map(UserDto::from).collect(),
        })
    }
}
