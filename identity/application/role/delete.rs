use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::UserRepository;
use crate::UserIdAndRole;

pub struct Delete<'a> {
    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Delete<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        Delete {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        role_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("delete_role") {
            return Err(Error::unauthorized());
        }

        let role_id = RoleId::new(role_id)?;

        let p_users = self
            .user_repo
            .search(Some(&role_id), None, None, None, Some(5), None)
            .await?;

        if p_users.total() > 0 {
            return Err(Error::new("role", "existing_users_assigned_to_role"));
        }

        let mut role = self.role_repo.find_by_id(&role_id).await?;

        if role.is_default() {
            return Err(Error::new("role", "is_default"));
        }

        role.delete()?;

        self.role_repo.delete(&role_id).await?;

        Ok(CommandResponse::default())
    }
}
