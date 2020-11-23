use serde::Deserialize;

use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository};
use crate::UserIdAndRole;

#[derive(Deserialize)]
pub struct SetFlagCommand {
    pub flag: i64,
}

pub struct SetFlag<'a> {
    user_repo: &'a dyn UserRepository,
}

impl<'a> SetFlag<'a> {
    pub fn new(user_repo: &'a dyn UserRepository) -> Self {
        SetFlag { user_repo }
    }

    pub async fn exec(
        &self,
        (auth_id, _auth_role): UserIdAndRole,
        user_id: String,
        cmd: SetFlagCommand,
    ) -> Result<CommandResponse> {
        let user_id = UserId::new(user_id)?;
        if auth_id != user_id {
            return Err(Error::unauthorized());
        }

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        user.set_flag(cmd.flag);

        self.user_repo.save(&mut user).await?;

        Ok(CommandResponse::default())
    }
}
