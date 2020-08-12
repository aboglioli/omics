use serde::Deserialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::application::util;
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
        util::is_authorized(auth_user, user_id)?;

        cmd.validate()?;

        let mut user = self.user_repo.find_by_id(&user_id).await?;

        let person = Person::new(Fullname::new(&cmd.name, &cmd.lastname)?)?;
        user.set_person(person)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn non_existing_user() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.user_repo());

        let user = mocks::user1();

        assert!(uc
            .exec(
                &user,
                &user.base().id(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                }
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn invalid_data() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(
                &user,
                &user.base().id(),
                UpdateCommand {
                    name: "N".to_owned(),
                    lastname: "L".to_owned(),
                }
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn valid_data() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(
                &user,
                &user.base().id(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                }
            )
            .await
            .is_ok());

        let saved_user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert_eq!(saved_user.person().unwrap().fullname().name(), "Name");
        assert_eq!(
            saved_user.person().unwrap().fullname().lastname(),
            "Lastname"
        );

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
