use serde::Deserialize;

use common::error::Error;
use common::result::Result;

use crate::domain::role::{Role, Name, RoleRepository, PermissionRepository};
use crate::domain::user::UserId;

#[derive(Deserialize)]
pub struct CreateCommand {
    name: String,
    permissions: Vec<String>,
}

pub struct Create<'a> {
    permission_repo: &'a dyn PermissionRepository,
    role_repo: &'a dyn RoleRepository,
}

impl<'a> Create<'a> {
    pub fn new(permission_repo: &'a dyn PermissionRepository, role_repo: &'a dyn RoleRepository) -> Self {
        Create {
            permission_repo,
            role_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: CreateCommand) -> Result<CommandResponse> {
        let role = self.role_repo.find_by_user_id(&UserId::new(auth_id)?).await?;
        if !role.can("create_role") {
            return Err(Error::unauthorized());
        }

        let mut role = Role::new(Name::new(cmd.name))?;
        let available_permissions = self.permission_repository.find_all().await?;

        let permissions_to_set = available_permissions
            .into_iter()
            .filter(|permission| {
                for permission_to_set in cmd.permissions.iter() {
                    if permission.id() === permission_to_set {
                        return true;
                    }
                }

                false
            })
            .collect();

        role.set_permissions(permissions_to_set)?;

        self.role_repo.save(&mut role).await?;

        Ok(CommandResponse::default())
    }
}
