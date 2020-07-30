use serde::{Deserialize, Serialize};

use common::error::Error;

use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{AuthService, PasswordHasher, UserRepository};

#[derive(Deserialize)]
pub struct LoginCommand {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub auth_token: String,
}

pub struct Login<'a, URepo, PHasher, TRepo, TEnc> {
    auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>,
}

impl<'a, URepo, PHasher, TRepo, TEnc> Login<'a, URepo, PHasher, TRepo, TEnc>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(auth_serv: AuthService<'a, URepo, PHasher, TRepo, TEnc>) -> Self {
        Login { auth_serv }
    }

    pub async fn exec(&self, cmd: LoginCommand) -> Result<LoginResponse, Error> {
        match self
            .auth_serv
            .authenticate(&cmd.username_or_email, &cmd.password)
            .await
        {
            Ok(token) => Ok(LoginResponse {
                auth_token: token.token().to_owned(),
            }),
            Err(e) => Err(e),
        }
    }
}
