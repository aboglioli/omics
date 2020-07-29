use std::sync::Arc;

use serde::Deserialize;

use common::error::Error;
use common::event::{EventPublisher, ToEvent};

use crate::domain::role::RoleId;
use crate::domain::token::{TokenEncoder, TokenRepository};
use crate::domain::user::{
    AuthService, Email, Identity, Password, PasswordHasher, Provider, User, UserEvent,
    UserRepository, Username,
};

#[derive(Deserialize)]
pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl RegisterCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Register<EPub, URepo, PHasher, TRepo, TEnc> {
    event_pub: Arc<EPub>,
    auth_serv: Arc<AuthService<URepo, PHasher, TRepo, TEnc>>,
    user_repo: Arc<URepo>,
}

impl<EPub, URepo, PHasher, TRepo, TEnc> Register<EPub, URepo, PHasher, TRepo, TEnc>
where
    EPub: EventPublisher,
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TRepo: TokenRepository,
    TEnc: TokenEncoder,
{
    pub fn new(
        event_pub: Arc<EPub>,
        auth_serv: Arc<AuthService<URepo, PHasher, TRepo, TEnc>>,
        user_repo: Arc<URepo>,
    ) -> Self {
        Register {
            event_pub,
            auth_serv,
            user_repo,
        }
    }

    pub async fn exec(&self, cmd: RegisterCommand) -> Result<(), Error> {
        cmd.validate()?;

        self.auth_serv.available(&cmd.username, &cmd.email).await?;
        let hashed_password = self.auth_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repo.next_id().await?,
            Identity::new(
                Provider::Local,
                Username::new(&cmd.username)?,
                Email::new(&cmd.email)?,
                Some(Password::new(&hashed_password)?),
            )?,
            RoleId::from("user"),
        )?;

        self.user_repo.save(&mut user).await?;

        let _event = UserEvent::Registered {
            id: user.base().id(),
            username: user.identity().username().value().to_owned(),
            email: user.identity().email().value().to_owned(),
        }
        .to_event()?;
        // self.event_pub.publish(event)?;

        Ok(())
    }
}
