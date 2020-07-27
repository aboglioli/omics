use std::sync::Arc;

use common::error::Error;
use common::event::EventPublisher;

use crate::domain::role::RoleId;
use crate::domain::user::{
    AuthService, Email, Identity, Password, Provider, User, UserRegistered, UserRepository,
    Username,
};

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

pub struct Register {
    auth_serv: Arc<AuthService>,
    event_pub: Arc<dyn EventPublisher<Output = usize>>,
    user_repo: Arc<dyn UserRepository>,
}

impl Register {
    pub fn new(
        auth_serv: Arc<AuthService>,
        event_pub: Arc<dyn EventPublisher<Output = usize>>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Register {
            auth_serv,
            event_pub,
            user_repo,
        }
    }

    pub fn exec(&self, cmd: RegisterCommand) -> Result<(), Error> {
        cmd.validate()?;

        self.auth_serv.available(&cmd.username, &cmd.email)?;
        let hashed_password = self.auth_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repo.next_id()?,
            Identity::new(
                Provider::Local,
                Username::new(&cmd.username)?,
                Email::new(&cmd.email)?,
                Some(Password::new(&hashed_password)?),
            )?,
            RoleId::from("user"),
        )?;

        self.user_repo.save(&mut user)?;

        let event = UserRegistered::new(
            user.base().id(),
            user.identity().username().value(),
            user.identity().email().value(),
        );
        self.event_pub.publish("user.registered", &event)?;

        Ok(())
    }
}
