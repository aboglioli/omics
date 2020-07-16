use crate::common::error::Error;
use crate::common::event::EventPublisher;
use crate::common::model::Entity;
use crate::identity::domain::user::{Person, User, UserID, UserRepository, UserUpdated};

pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
}

impl UpdateCommand {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct UserService<'a, UR, EP> {
    user_repository: &'a UR,
    event_publisher: &'a EP,
}

impl<'a, UR, EP> UserService<'a, UR, EP>
where
    UR: UserRepository,
    EP: EventPublisher,
{
    pub fn new<'b>(user_repository: &'b UR, event_publisher: &'b EP) -> UserService<'b, UR, EP> {
        UserService {
            user_repository,
            event_publisher,
        }
    }

    pub fn get_by_id(&self, user_id: UserID) -> Result<User, Error> {
        let user = self.user_repository.find_by_id(user_id)?;
        Ok(user)
    }

    pub fn update(&self, user_id: UserID, cmd: UpdateCommand) -> Result<(), Error> {
        cmd.validate()?;

        let mut user = self.user_repository.find_by_id(user_id)?;

        user.change_name(&cmd.name, &cmd.lastname)?;

        self.user_repository.save(&mut user)?;

        if let Some(person) = user.person() {
            let event = UserUpdated::new(user.id().value(), person.clone());
            self.event_publisher.publish("user.updated", event)?;
        }

        Ok(())
    }
}
