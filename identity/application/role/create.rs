use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

use crate::domain::role::{Name, PermissionRepository, Role, RoleRepository};

use crate::UserIdAndRole;

#[derive(Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: String,
}

pub struct Create<'a> {
    permission_repo: &'a dyn PermissionRepository,
    role_repo: &'a dyn RoleRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        permission_repo: &'a dyn PermissionRepository,
        role_repo: &'a dyn RoleRepository,
    ) -> Self {
        Create {
            permission_repo,
            role_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        cmd: CreateCommand,
    ) -> Result<CreateResponse> {
        if !auth_role.can("create_role") {
            return Err(Error::unauthorized());
        }

        let mut role = Role::new(Name::new(cmd.name)?)?;

        if self.role_repo.find_by_id(role.base().id()).await.is_ok() {
            return Err(Error::new("role", "already_exists"));
        }

        let available_permissions = self.permission_repo.find_all().await?;

        let permissions = &cmd.permissions;
        let permissions_to_set = available_permissions
            .into_iter()
            .filter(|permission| {
                for permission_to_set in permissions.iter() {
                    if permission.id() == permission_to_set {
                        return true;
                    }
                }

                false
            })
            .collect();

        role.set_permissions(permissions_to_set)?;

        self.role_repo.save(&mut role).await?;

        Ok(CreateResponse {
            id: role.base().id().to_string(),
        })
    }
}
