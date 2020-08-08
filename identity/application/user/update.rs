use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::user::{Fullname, Person, User, UserId, UserRepository};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    lastname: String,
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

    pub async fn exec(&self, auth_user: &User, user_id: &UserId, cmd: UpdateCommand) -> Result<()> {
        authorized(auth_user, user_id)?;

        cmd.validate()?;

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        let person = Person::new(Fullname::new(&cmd.name, &cmd.lastname)?)?;
        user.set_person(person)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}

fn authorized(auth_user: &User, user_id: &UserId) -> Result<()> {
    let guard = &auth_user.base().id() == user_id || auth_user.role().base().id() == "admin";

    if !guard {
        return Err(Error::new("user", "unauthorized"));
    }

    Ok(())
}
