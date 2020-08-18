use serde::Deserialize;

use common::result::Result;

use crate::domain::role::{RoleId, RoleRepository};
use crate::domain::user::{UserId, UserRepository};

#[derive(Deserialize)]
pub struct ChangeRoleCommand {
    pub role_id: String,
}

pub struct ChangeRole<'a, RRepo, URepo> {
    role_repo: &'a RRepo,
    user_repo: &'a URepo,
}

impl<'a, RRepo, URepo> ChangeRole<'a, RRepo, URepo>
where
    URepo: UserRepository,
    RRepo: RoleRepository,
{
    pub fn new(role_repo: &'a RRepo, user_repo: &'a URepo) -> Self {
        ChangeRole {
            role_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        admin_id: String,
        user_id: String,
        cmd: ChangeRoleCommand,
    ) -> Result<()> {
        let admin = self.user_repo.find_by_id(&UserId::new(admin_id)?).await?;
        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;
        let role = self
            .role_repo
            .find_by_id(&RoleId::new(cmd.role_id)?)
            .await?;

        user.change_role(role, &admin)?;

        Ok(())
    }
}
