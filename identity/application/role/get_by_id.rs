use common::error::Error;

use common::result::Result;

use crate::application::dtos::RoleDto;
use crate::domain::role::{RoleId, RoleRepository};
use crate::UserIdAndRole;

pub struct GetById<'a> {
    role_repo: &'a dyn RoleRepository,
}

impl<'a> GetById<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository) -> Self {
        GetById { role_repo }
    }

    pub async fn exec(&self, (_auth_id, auth_role): UserIdAndRole, id: String) -> Result<RoleDto> {
        if !auth_role.can("get_any_role") {
            if auth_role.base().id().value() != id || !auth_role.can("get_own_role") {
                return Err(Error::unauthorized());
            }
        }

        let role = self.role_repo.find_by_id(&RoleId::new(id)?).await?;

        Ok(RoleDto::from(&role))
    }
}
