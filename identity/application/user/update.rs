use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::application::dtos::CommandResponse;
use crate::domain::user::{Fullname, Person, UserId, UserRepository};

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
}

pub struct Update<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,
}

impl<'a> Update<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, user_repo: &'a dyn UserRepository) -> Self {
        Update {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        user_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if auth_id != user_id {
            let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            if !auth_user.role().is("admin") {
                return Err(Error::unauthorized());
            }
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

        let person = Person::new(Fullname::new(cmd.name, cmd.lastname)?)?;
        user.set_person(person)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.base().events()?).await?;

        Ok(CommandResponse::default())
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
                user.base().id().to_string(),
                user.base().id().to_string(),
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
                user.base().id().to_string(),
                user.base().id().to_string(),
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
                user.base().id().to_string(),
                user.base().id().to_string(),
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
