use common::event::EventPublisher;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository, Validation};

pub struct Validate<'a, URepo, EPub> {
    event_pub: &'a EPub,

    user_repo: &'a URepo,
}

impl<'a, URepo, EPub> Validate<'a, URepo, EPub>
where
    EPub: EventPublisher,
    URepo: UserRepository,
{
    pub fn new(event_pub: &'a EPub, user_repo: &'a URepo) -> Self {
        Validate {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: &UserId, validation: &Validation) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        user.validate(validation)?;

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
    async fn invalid_code() {
        let c = mocks::container();
        let uc = Validate::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .exec(&user.base().id(), &Validation::new())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn valid_code() {
        let c = mocks::container();
        let uc = Validate::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();
        assert!(!user.is_validated());

        assert!(uc
            .exec(&user.base().id(), user.validation().unwrap())
            .await
            .is_ok());

        let saved_user = c.user_repo().find_by_id(&user.base().id()).await.unwrap();
        assert!(saved_user.is_validated());

        assert!(uc
            .exec(&user.base().id(), user.validation().unwrap())
            .await
            .is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }
}
