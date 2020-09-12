use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::user::{
    Biography, Birthdate, Fullname, Gender, Image, Person, UserId, UserRepository,
};

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub name: String,
    pub lastname: String,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub profile_image: Option<String>,
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
            if !auth_user.is_admin() {
                return Err(Error::unauthorized());
            }
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

        let birthdate = cmd
            .birthdate
            .map(|date| Birthdate::from_str(&date))
            .transpose()?;

        let gender = cmd.gender.map(|gender| Gender::from(&gender)).transpose()?;

        let biography = cmd
            .biography
            .map(|biography| Biography::new(biography))
            .transpose()?;

        let profile_image = cmd
            .profile_image
            .map(|image| Image::new(image))
            .transpose()?;

        let person = Person::new(
            Fullname::new(cmd.name, cmd.lastname)?,
            birthdate,
            gender,
            biography,
            profile_image,
        )?;
        user.set_person(person)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;
    use chrono::Datelike;

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
                    birthdate: None,
                    gender: None,
                    biography: None,
                    profile_image: None,
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
                    birthdate: None,
                    gender: None,
                    biography: None,
                    profile_image: None,
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.base().id().to_string(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                    birthdate: Some("invalid-date".to_owned()),
                    gender: None,
                    biography: None,
                    profile_image: None,
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.base().id().to_string(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                    birthdate: Some("1996-12-32T24:39:57-08:00".to_owned()),
                    gender: None,
                    biography: None,
                    profile_image: None,
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.base().id().to_string(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                    birthdate: None,
                    gender: Some("malee".to_owned()),
                    biography: None,
                    profile_image: None,
                }
            )
            .await
            .is_err());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.base().id().to_string(),
                UpdateCommand {
                    name: "Name".to_owned(),
                    lastname: "Lastname".to_owned(),
                    birthdate: None,
                    gender: Some("female2".to_owned()),
                    biography: None,
                    profile_image: None,
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
                    birthdate: Some("1996-12-19T23:39:57-03:00".to_owned()),
                    gender: Some("male".to_owned()),
                    biography: Some("My amazing biography".to_owned()),
                    profile_image: Some("http://profile.com/profile.jpg".to_owned()),
                }
            )
            .await
            .is_ok());

        let saved_user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        let person = saved_user.person().unwrap();

        assert_eq!(
            person.profile_image().unwrap().url(),
            "http://profile.com/profile.jpg"
        );

        assert_eq!(person.fullname().name(), "Name");
        assert_eq!(person.fullname().lastname(), "Lastname");
        assert!(person.birthdate().is_some());
        assert!(person.gender().is_some());
        assert!(person.biography().is_some());
        assert!(person.profile_image().is_some());

        let birthdate = person.birthdate().unwrap().date();
        assert_eq!(birthdate.to_rfc3339(), "1996-12-20T02:39:57+00:00");
        assert_eq!(birthdate.year(), 1996);
        assert_eq!(birthdate.month(), 12);
        assert_eq!(birthdate.day(), 20);
        assert!(matches!(person.gender().unwrap(), Gender::Male));

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
