use std::sync::Arc;

use common::error::Error;

use crate::domain::user::AuthService;

pub struct LoginCommand {
    pub username_or_email: String,
    pub password: String,
}

pub struct LoginResponse {
    pub auth_token: String,
}

pub struct Login {
    auth_serv: Arc<AuthService>,
}

impl Login {
    pub fn new(auth_serv: Arc<AuthService>) -> Self {
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
