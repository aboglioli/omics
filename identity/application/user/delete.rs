use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
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

    pub async fn exec(&self, auth_id: String, user_id: String) -> Result<CommandResponse> {
        if auth_id != user_id {
            let auth_user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            if !auth_user.role().is("admin") {
                return Err(Error::unauthorized());
            }
        }

        let mut user = self.user_repo.find_by_id(&UserId::new(user_id)?).await?;

        user.delete()?;

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
    async fn delete() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::validated_user1();
        c.user_repo().save(&mut user).await.unwrap();

        let user_id = user.base().id().to_string();
        assert!(uc.exec(user_id.clone(), user_id.clone()).await.is_ok());
        assert!(uc.exec(user_id.clone(), user_id).await.is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn not_validated() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        let user_id = user.base().id().to_string();
        assert!(uc.exec(user_id.clone(), user_id).await.is_err());
    }
}
