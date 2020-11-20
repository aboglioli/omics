use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::RoleDto;
use crate::domain::role::RoleRepository;
use crate::UserIdAndRole;

#[derive(Serialize)]
pub struct GetAllResponse {
    pub roles: Vec<RoleDto>,
}

pub struct GetAll<'a> {
    role_repo: &'a dyn RoleRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository) -> Self {
        GetAll { role_repo }
    }

    pub async fn exec(&self, (_auth_id, auth_role): UserIdAndRole) -> Result<GetAllResponse> {
        if !auth_role.can("get_all_roles") {
            return Err(Error::unauthorized());
        }

        let roles = self.role_repo.find_all().await?;

        Ok(GetAllResponse {
            roles: roles.iter().map(RoleDto::from).collect(),
        })
    }
}
