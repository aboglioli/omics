use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use common::error::Error;

use crate::domain::user::{Email, User, UserId, UserRepository, Username};

pub struct InMemUserRepository {
    pub users: Mutex<HashMap<UserId, User>>,
}

impl InMemUserRepository {
    pub fn new() -> InMemUserRepository {
        InMemUserRepository {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemUserRepository {
    async fn next_id(&self) -> Result<UserId, Error> {
        let uuid = Uuid::new_v4();
        let uuid = uuid.to_string();
        Ok(uuid)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User, Error> {
        let users = self.users.lock().unwrap();
        users
            .get(id)
            .cloned()
            .ok_or(Error::internal().set_code("not_found").build())
    }

    async fn find_by_username(&self, username: &Username) -> Result<User, Error> {
        for (_, user) in self.users.lock().unwrap().iter() {
            if user.identity().username().value() == username.value() {
                return Ok(user.clone());
            }
        }
        Err(Error::internal().set_code("not_found").build())
    }

    async fn find_by_email(&self, email: &Email) -> Result<User, Error> {
        for (_, user) in self.users.lock().unwrap().iter() {
            if user.identity().email().value() == email.value() {
                return Ok(user.clone());
            }
        }
        Err(Error::internal().set_code("not_found").build())
    }

    async fn save(&self, user: &mut User) -> Result<(), Error> {
        // user.id().updated();
        self.users
            .lock()
            .unwrap()
            .insert(user.base().id(), user.clone());
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use tokio;
//
//     use crate::domain::user::*;
//     use crate::infrastructure::mocks;
//
//     #[tokio::test]
//     async fn next_id() -> Result<(), Error> {
//         let repo = InMemUserRepository::new();
//         let id1 = repo.next_id().await?;
//         let id2 = repo.next_id().await?;
//         let id3 = repo.next_id().await?;
//         assert!(id1.len() > 10);
//         assert_ne!(id1, id2);
//         assert_ne!(id2, id3);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     fn find_by_id() -> Result<(), Error> {
//         let repo = InMemUserRepository::new();
//         let user = mocks::user1()?;
//         let mut changed_user = user.clone();
//         changed_user.set_person(Person::new(Fullname::new("Name", "Lastname")?)?)?;
//
//         repo.save(&mut changed_user)?;
//         assert_eq!(repo.users.lock().unwrap().len(), 1);
//         assert!(user.person().is_none());
//
//         let found_user = repo.find_by_id(&user.base().id())?;
//         assert_eq!(user.base(), found_user.base());
//         assert_eq!(changed_user.base(), found_user.base());
//
//         let changed_user_person = found_user.person().ok_or(Error::internal())?;
//         assert_eq!(changed_user_person.fullname().name(), "Name");
//         assert_eq!(changed_user_person.fullname().lastname(), "Lastname");
//
//         assert!(repo
//             .find_by_username(&Username::new("username").unwrap())
//             .is_ok());
//         assert!(repo
//             .find_by_email(&Email::new("username@email.com").unwrap())
//             .is_ok());
//         assert!(repo
//             .find_by_username(&Username::new("nonexisting").unwrap())
//             .is_err());
//         assert!(repo
//             .find_by_email(&Email::new("username@asd.com").unwrap())
//             .is_err());
//
//         Ok(())
//     }
// }
