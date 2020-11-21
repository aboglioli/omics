use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::role::{RoleId, RoleRepository};
use crate::UserIdAndRole;

pub struct MakeDefault<'a> {
    role_repo: &'a dyn RoleRepository,
}

impl<'a> MakeDefault<'a> {
    pub fn new(role_repo: &'a dyn RoleRepository) -> Self {
        MakeDefault { role_repo }
    }

    pub async fn exec(
        &self,
        (_auth_id, auth_role): UserIdAndRole,
        role_id: String,
    ) -> Result<CommandResponse> {
        if !auth_role.can("make_role_default") {
            return Err(Error::unauthorized());
        }

        let mut role = self.role_repo.find_by_id(&RoleId::new(role_id)?).await?;
        let mut default_role = self.role_repo.find_default().await?;

        if role.base().id() == default_role.base().id() {
            return Ok(CommandResponse::default());
        }

        role.set_default(true)?;
        default_role.set_default(false)?;

        self.role_repo.save(&mut role).await?;
        self.role_repo.save(&mut default_role).await?;

        Ok(CommandResponse::default())
    }
}
