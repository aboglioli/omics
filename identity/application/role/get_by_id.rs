use common::error::Error;

use common::result::Result;

use crate::application::dtos::RoleDto;
use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserRepository};
use crate::UserIdAndRole;

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

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole, id: String) -> Result<RoleDto> {
        let user = self.user_repo.find_by_id(&auth_id).await?;
        if !auth_role.can("get_all_roles") {
            if user.role_id().value() != id {
                return Err(Error::unauthorized());
            } else if !auth_role.can("get_own_role") {
                return Err(Error::unauthorized());
            }
        }

        let role = self.role_repo.find_by_id(&RoleId::new(id)?).await?;

        Ok(RoleDto::from(&role))
    }
}
