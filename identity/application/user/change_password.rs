use std::sync::Arc;

use common::error::Error;

use crate::domain::user::{AuthService, UserId};

pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ChangePassword {
    auth_serv: Arc<AuthService>,
}

impl ChangePassword {
    pub fn new(auth_serv: Arc<AuthService>) -> Self {
        ChangePassword { auth_serv }
    }

    pub fn exec(&self, user_id: &UserId, cmd: ChangePasswordCommand) -> Result<(), Error> {
        cmd.validate()?;
        self.auth_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
    }
}
