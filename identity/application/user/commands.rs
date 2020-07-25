use crate::domain::role::RoleId;
use common::error::Error;

pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct LoginCommand {
    pub username_or_email: String,
    pub password: String,
}

pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: RoleId,
}

impl RegisterCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
