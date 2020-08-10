use serde::Deserialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::role::{Role, RoleId};

use crate::domain::user::{
    Email, Identity, Password, PasswordHasher, Provider, User, UserRepository, UserService,
    Username,
};

#[derive(Deserialize)]
pub struct RegisterCommand {
    username: String,
    email: String,
    password: String,
}

impl RegisterCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Register<'a, EPub, URepo, PHasher> {
    event_pub: &'a EPub,

    user_repo: &'a URepo,

    user_serv: UserService<'a, URepo, PHasher>,
}

impl<'a, EPub, URepo, PHasher> Register<'a, EPub, URepo, PHasher>
where
    EPub: EventPublisher,
    URepo: UserRepository,
    PHasher: PasswordHasher,
{
    pub fn new(
        event_pub: &'a EPub,
        user_repo: &'a URepo,
        user_serv: UserService<'a, URepo, PHasher>,
    ) -> Self {
        Register {
            event_pub,
            user_repo,
            user_serv,
        }
    }

    pub async fn exec(&self, cmd: RegisterCommand) -> Result<()> {
        cmd.validate()?;

        self.user_serv.available(&cmd.username, &cmd.email).await?;
        let hashed_password = self.user_serv.generate_password(&cmd.password)?;

        let mut user = User::new(
            self.user_repo.next_id().await?,
            Identity::new(
                Provider::Local,
                Username::new(&cmd.username)?,
                Email::new(&cmd.email)?,
                Some(Password::new(&hashed_password)?),
            )?,
            Role::new(RoleId::new("user")?, "User")?,
        )?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}
