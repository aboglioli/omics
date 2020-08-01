use common::event::{EventPublisher, ToEvent};
use common::result::Result;

use crate::domain::user::{Fullname, Person, UserEvent, UserId, UserRepository};

pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Update<'a, EPub, URepo> {
    event_pub: &'a EPub,
    user_repo: &'a URepo,
}

impl<'a, EPub, URepo> Update<'a, EPub, URepo>
where
    EPub: EventPublisher,
    URepo: UserRepository,
{
    pub fn new(event_pub: &'a EPub, user_repo: &'a URepo) -> Self {
        Update {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: &UserId, cmd: UpdateCommand) -> Result<()> {
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
            self.event_pub.publish(event).await?;
        }

        Ok(())
    }
}
