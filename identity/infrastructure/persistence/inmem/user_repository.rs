use async_trait::async_trait;
use uuid::Uuid;

use common::cache::{inmem::InMemCache, Cache};
use common::result::Result;

use crate::domain::user::{Email, User, UserId, UserRepository, Username};

pub struct InMemUserRepository {
    cache: InMemCache<UserId, User>,
}

impl InMemUserRepository {
    pub fn new() -> Self {
        InMemUserRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemUserRepository {
    async fn next_id(&self) -> Result<UserId> {
        let uuid = Uuid::new_v4();
        let uuid = uuid.to_string();
        UserId::new(&uuid)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User> {
        self.cache.get(id).await.ok_or_else(Self::err_not_found)
    }

    async fn find_by_username(&self, username: &Username) -> Result<User> {
        self.cache
            .find(|(_, user)| user.identity().username().value() == username.value())
            .await
            .ok_or_else(Self::err_not_found)
    }

    async fn find_by_email(&self, email: &Email) -> Result<User> {
        self.cache
            .find(|(_, user)| user.identity().email().value() == email.value())
            .await
            .ok_or_else(Self::err_not_found)
    }

    async fn save(&self, user: &mut User) -> Result<()> {
        self.cache.set(user.base().id(), user.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::user::*;
    use crate::infrastructure::mocks;

    #[tokio::test]
    async fn next_id() {
        let repo = InMemUserRepository::new();
        let id1 = repo.next_id().await.unwrap();
        let id2 = repo.next_id().await.unwrap();
        let id3 = repo.next_id().await.unwrap();
        assert!(id1.value().len() > 10);
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
    }

    #[tokio::test]
    async fn find_by_id() {
        let repo = InMemUserRepository::new();
        let user = mocks::user1();
        let mut changed_user = user.clone();
        changed_user
            .set_person(Person::new(Fullname::new("Name", "Lastname").unwrap()).unwrap())
            .unwrap();

        repo.save(&mut changed_user).await.unwrap();
        assert!(repo.find_by_id(&changed_user.base().id()).await.is_ok());
        assert!(user.person().is_none());

        let found_user = repo.find_by_id(&user.base().id()).await.unwrap();
        assert_eq!(user.base(), found_user.base());
        assert_eq!(changed_user.base(), found_user.base());

        let changed_user_person = found_user.person().unwrap();
        assert_eq!(changed_user_person.fullname().name(), "Name");
        assert_eq!(changed_user_person.fullname().lastname(), "Lastname");

        assert!(repo
            .find_by_username(&Username::new("username").unwrap())
            .await
            .is_ok());
        assert!(repo
            .find_by_email(&Email::new("username@email.com").unwrap())
            .await
            .is_ok());
        assert!(repo
            .find_by_username(&Username::new("nonexisting").unwrap())
            .await
            .is_err());
        assert!(repo
            .find_by_email(&Email::new("username@asd.com").unwrap())
            .await
            .is_err());
    }
}
