use serde::Deserialize;

use common::result::Result;

use crate::domain::user::{PasswordHasher, UserId, UserRepository, UserService};

#[derive(Deserialize)]
pub struct ChangePasswordCommand {
    old_password: String,
    new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct ChangePassword<'a, URepo, PHasher> {
    user_serv: UserService<'a, URepo, PHasher>,
}

impl<'a, URepo, PHasher> ChangePassword<'a, URepo, PHasher>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
{
    pub fn new(user_serv: UserService<'a, URepo, PHasher>) -> Self {
        ChangePassword { user_serv }
    }

    pub async fn exec(&self, user_id: &UserId, cmd: ChangePasswordCommand) -> Result<()> {
        cmd.validate()?;
        self.user_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
            .await
    }
}
