use serde::Deserialize;

use common::error::Error;
use common::result::Result;

use crate::domain::user::{PasswordHasher, User, UserId, UserRepository, UserService};

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

    pub async fn exec(
        &self,
        auth_user: &User,
        user_id: &UserId,
        cmd: ChangePasswordCommand,
    ) -> Result<()> {
        authorized(auth_user, user_id)?;

        cmd.validate()?;

        self.user_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
            .await
    }
}

fn authorized(auth_user: &User, user_id: &UserId) -> Result<()> {
    let guard = &auth_user.base().id() == user_id || auth_user.role().base().id() == "admin";

    if !guard {
        return Err(Error::new("user", "unauthorized"));
    }

    Ok(())
}
