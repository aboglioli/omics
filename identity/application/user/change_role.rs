use serde::Deserialize;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;
use common::event::EventPublisher;

use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserId, UserRepository};

#[derive(Deserialize)]
pub struct ChangeRoleCommand {
    pub role_id: String,
}

pub struct ChangeRole<'a> {
    event_pub: &'a dyn EventPublisher,

    role_repo: &'a dyn RoleRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> ChangeRole<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, role_repo: &'a dyn RoleRepository, user_repo: &'a dyn UserRepository) -> Self {
        ChangeRole {
            event_pub,
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

        if !admin.is_admin() {
            return Err(Error::unauthorized());
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;
        let role = self
            .role_repo
            .find_by_id(&RoleId::new(cmd.role_id)?)
            .await?;

        user.change_role(role.base().id().clone())?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}
