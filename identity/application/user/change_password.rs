use common::result::Result;

use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{AuthService, PasswordHasher, UserId, UserRepository};

pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct ChangePassword<'a, URepo, PHasher, TRepo, TEnc> {
    auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>,
}

impl<'a, URepo, PHasher, TRepo, TEnc> ChangePassword<'a, URepo, PHasher, TRepo, TEnc>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>) -> Self {
        ChangePassword { auth_serv }
    }

    pub async fn exec(&self, user_id: &UserId, cmd: ChangePasswordCommand) -> Result<()> {
        cmd.validate()?;
        self.auth_serv
            .change_password(user_id, &cmd.old_password, &cmd.new_password)
            .await
    }
}
