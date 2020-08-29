use common::error::Error;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{RoleDto, UserDto};
use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserId, UserRepository};

pub struct GetById<'a> {
    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetById<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        GetById {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, id: String, include: Include) -> Result<RoleDto> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        if !user.role().is("admin") {
            return Err(Error::unauthorized());
        }

        let role = self.role_repo.find_by_id(&RoleId::new(id)?).await?;
        let mut role_dto = RoleDto::from(&role);

        if include.has("users") {
            role_dto = role_dto.users(
                self.user_repo
                    .find_by_role_id(role.base().id())
                    .await?
                    .iter()
                    .map(UserDto::from)
                    .collect(),
            );
        }

        Ok(role_dto)
    }
}
