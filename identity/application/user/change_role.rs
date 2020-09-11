use serde::Deserialize;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserId, UserRepository};

#[derive(Deserialize)]
pub struct ChangeRoleCommand {
    pub role_id: String,
}

// TODO: emit event
pub struct ChangeRole<'a> {
    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> ChangeRole<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        ChangeRole {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        user_id: String,
        cmd: ChangeRoleCommand,
    ) -> Result<CommandResponse> {
        let admin = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;

        if admin.role_id().value() != "admin" {
            return Err(Error::unauthorized());
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;
        let role = self
            .role_repo
            .find_by_id(&RoleId::new(cmd.role_id)?)
            .await?;

        user.change_role(role.base().id().clone())?;

        self.user_repo.save(&mut user).await?;

        Ok(CommandResponse::default())
    }
}
