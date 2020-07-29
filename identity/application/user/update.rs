use std::sync::Arc;

use common::error::Error;
use common::event::{EventPublisher, ToEvent};

use crate::domain::user::{Fullname, Person, UserEvent, UserId, UserRepository};

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
    event_pub: Arc<dyn EventPublisher<Output = usize>>,
    user_repo: Arc<dyn UserRepository>,
}

impl Update {
    pub fn new(
        event_pub: Arc<dyn EventPublisher<Output = usize>>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Update {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: &UserId, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        let person = Person::new(Fullname::new(&cmd.name, &cmd.lastname)?)?;
        user.set_person(person)?;
        self.user_repo.save(&mut user).await?;

        if let Some(person) = user.person() {
            let event = UserEvent::Updated {
                id: user.base().id(),
                name: person.fullname().name().to_owned(),
                lastname: person.fullname().lastname().to_owned(),
            }
            .to_event()?;
            self.event_pub.publish(event)?;
        }

        Ok(())
    }
}
