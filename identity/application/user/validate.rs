use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository, Validation};

pub struct Validate<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,
}

impl<'a> Validate<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, user_repo: &'a dyn UserRepository) -> Self {
        Validate {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: String, validation_code: String) -> Result<CommandResponse> {
        let user_id = UserId::new(user_id)?;
        let mut user = self.user_repo.find_by_id(&user_id).await?;

        let validation = Validation::from(validation_code);
        user.validate(&validation)?;

        self.user_repo.save(&mut user).await?;

        self.event_pub.publish_all(user.events().to_vec()?).await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn invalid_code() {
        let c = mocks::container();
        let uc = Validate::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            false,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(user.base().id().to_string(), "invalid-123".to_owned())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn valid_code() {
        let c = mocks::container();
        let uc = Validate::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user(
            "user-1",
            "username",
            "user@omics.com",
            "P@asswd!",
            false,
            None,
            None,
            "user",
        );
        c.user_repo().save(&mut user).await.unwrap();
        assert!(!user.is_validated());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.validation().unwrap().code().to_string()
            )
            .await
            .is_ok());

        let saved_user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert!(saved_user.is_validated());

        assert!(uc
            .exec(
                user.base().id().to_string(),
                user.validation().unwrap().code().to_string()
            )
            .await
            .is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
