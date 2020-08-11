use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{AuthenticationService, PasswordHasher, UserRepository};

#[derive(Deserialize)]
pub struct LoginCommand {
    username_or_email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub auth_token: String,
}

pub struct Login<'a, EPub, URepo, PHasher, TRepo, TEnc> {
    event_pub: &'a EPub,

    authentication_serv: AuthenticationService<'a, URepo, PHasher, TRepo, TEnc>,
}

impl<'a, EPub, URepo, PHasher, TRepo, TEnc> Login<'a, EPub, URepo, PHasher, TRepo, TEnc>
where
    EPub: EventPublisher,
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        event_pub: &'a EPub,
        authentication_serv: AuthenticationService<'a, URepo, PHasher, TRepo, TEnc>,
    ) -> Self {
        Login {
            event_pub,
            authentication_serv,
        }
    }

    pub async fn exec(&self, cmd: LoginCommand) -> Result<LoginResponse> {
        match self
            .authentication_serv
            .authenticate(&cmd.username_or_email, &cmd.password)
            .await
        {
            Ok((mut user, token)) => {
                user.login(&token)?;

                self.event_pub.publish_all(user.base().events()?).await?;

                Ok(LoginResponse {
                    auth_token: token.value().to_owned(),
                })
            }
            Err(e) => Err(e),
        }
    }
}
