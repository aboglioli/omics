use serde::Deserialize;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::{Name, PermissionRepository, RoleId, RoleRepository};

use crate::UserIdAndRole;

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
}

pub struct Update<'a> {
    permission_repo: &'a dyn PermissionRepository,
    role_repo: &'a dyn RoleRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        permission_repo: &'a dyn PermissionRepository,
        role_repo: &'a dyn RoleRepository,
    ) -> Self {
        Update {
            permission_repo,
            role_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        role_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("update_role") {
            return Err(Error::unauthorized());
        }

        let mut role = self.role_repo.find_by_id(&RoleId::new(role_id)?).await?;

        if let Some(name) = cmd.name {
            role.set_name(Name::new(name)?)?;
        }

        if let Some(permissions) = cmd.permissions {
            let available_permissions = self.permission_repo.find_all().await?;

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
        }

        self.role_repo.save(&mut role).await?;

        Ok(CommandResponse::default())
    }
}
