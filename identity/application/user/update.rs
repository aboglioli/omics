use std::sync::Arc;

use common::error::Error;
use common::event::EventPublisher;

use crate::domain::user::{
    AuthService, Email, Fullname, Identity, Password, Person, Provider, User, UserId,
    UserRepository, UserUpdated, Username,
};

pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Update {
    auth_serv: Arc<AuthService>,
    event_pub: Arc<dyn EventPublisher<Output = usize>>,
    user_repo: Arc<dyn UserRepository>,
}

impl Update {
    pub fn new(
        auth_serv: Arc<AuthService>,
        event_pub: Arc<dyn EventPublisher<Output = usize>>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Update {
            auth_serv,
            event_pub,
            user_repo,
        }
    }

    pub fn exec(&self, user_id: &UserId, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repo.find_by_id(&user_id)?;

        let person = Person::new(Fullname::new(&cmd.name, &cmd.lastname)?)?;
        user.set_person(person);
        self.user_repo.save(&mut user)?;

        if let Some(person) = user.person() {
            let event = UserUpdated::new(
                user.base().id(),
                person.fullname().name(),
                person.fullname().lastname(),
            );
            self.event_pub.publish("user.updated", &event)?;
        }

        Ok(())
    }
}
