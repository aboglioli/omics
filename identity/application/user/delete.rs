use common::event::EventPublisher;
use common::result::Result;

use crate::application::util;
use crate::domain::user::{User, UserId, UserRepository};

pub struct Delete<'a, EPub, URepo> {
    event_pub: &'a EPub,

    user_repo: &'a URepo,
}

impl<'a, EPub, URepo> Delete<'a, EPub, URepo>
where
    EPub: EventPublisher,
    URepo: UserRepository,
{
    pub fn new(event_pub: &'a EPub, user_repo: &'a URepo) -> Self {
        Delete {
            event_pub,
            user_repo,
        }
    }

    pub async fn exec(&self, auth_user: &User, user_id: &UserId) -> Result<()> {
        util::is_authorized(auth_user, user_id)?;

        let mut user = self.user_repo.find_by_id(user_id).await?;

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

        assert!(uc.exec(&user, &user.base().id()).await.is_ok());
        assert!(uc.exec(&user, &user.base().id()).await.is_err());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn not_validated() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();
        assert!(uc.exec(&user, &user.base().id()).await.is_err());
    }

    #[tokio::test]
    async fn not_owner_or_existing_user() {
        let c = mocks::container();
        let uc = Delete::new(c.event_pub(), c.user_repo());

        let mut user = mocks::user1();
        assert!(uc.exec(&user, &user.base().id()).await.is_err());

        c.user_repo().save(&mut user).await.unwrap();
        let other = mocks::user2();
        assert!(uc.exec(&other, &user.base().id()).await.is_err());
    }
}
