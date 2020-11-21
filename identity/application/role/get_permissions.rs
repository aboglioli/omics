use serde::Serialize;

use common::error::Error;
use common::result::Result;

use crate::application::dtos::PermissionDto;
use crate::domain::role::PermissionRepository;
use crate::UserIdAndRole;

#[derive(Serialize)]
pub struct GetPermissionsResponse {
    pub permissions: Vec<PermissionDto>,
}

pub struct GetPermissions<'a> {
    permission_repo: &'a dyn PermissionRepository,
}

impl<'a> GetPermissions<'a> {
    pub fn new(permission_repo: &'a dyn PermissionRepository) -> Self {
        GetPermissions { permission_repo }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
    ) -> Result<GetPermissionsResponse> {
        if !auth_role.can("get_permissions") {
            return Err(Error::unauthorized());
        }

        let permissions = self.permission_repo.find_all().await?;

        Ok(GetPermissionsResponse {
            permissions: permissions.iter().map(PermissionDto::from).collect(),
        })
    }
}
