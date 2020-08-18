use common::event::EventPublisher;
use common::result::Result;

use crate::domain::user::{UserId, UserRepository};

pub struct Delete<'a> {
    event_pub: &'a dyn EventPublisher,

    user_repo: &'a dyn UserRepository,
}

impl<'a> Delete<'a> {
    pub fn new(event_pub: &'a dyn EventPublisher, user_repo: &'a dyn UserRepository) -> Self {
        Delete {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, user_id: String) -> Result<()> {
        let user_id = UserId::new(user_id)?;
        let mut user = self.user_repo.find_by_id(&user_id).await?;

        user.delete()?;

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
    async fn delete() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::validated_user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc.exec(user.base().id().value().to_owned()).await.is_ok());
        assert!(uc.exec(user.base().id().value().to_owned()).await.is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn not_validated() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();
        assert!(uc.exec(user.base().id().value().to_owned()).await.is_err());
    }
}
