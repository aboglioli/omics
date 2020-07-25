use std::cell::RefCell;
use std::collections::HashMap;

use uuid::Uuid;

use common::error::Error;

use crate::domain::user::{User, UserId, UserRepository};

pub struct InMemUserRepository {
    pub users: RefCell<HashMap<UserId, User>>,
}

impl InMemUserRepository {
    pub fn new() -> InMemUserRepository {
        InMemUserRepository {
            users: RefCell::new(HashMap::new()),
        }
    }
}

impl UserRepository for InMemUserRepository {
    fn next_id(&self) -> Result<UserId, Error> {
        let uuid = Uuid::new_v4();
        let uuid = uuid.to_string();
        Ok(uuid)
    }

    fn find_by_id(&self, id: &UserId) -> Result<User, Error> {
        let users = self.users.borrow();
        users
            .get(id)
            .cloned()
            .ok_or(Error::internal().set_code("not_found").build())
    }

    fn find_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error> {
        // TODO: can be made functional. Don't be lazy.
        for (_, user) in self.users.borrow().iter() {
            if user.identity().username().value() == username_or_email
                || user.identity().email().value() == username_or_email
            {
                return Ok(user.clone());
            }
        }
        Err(Error::internal().set_code("not_found").build())
    }

    fn save(&self, user: &mut User) -> Result<(), Error> {
        // user.id().updated();
        self.users
            .borrow_mut()
            .insert(user.base().id(), user.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::role::{Role, RoleId};
    use crate::domain::user::{
        Email, Fullname, Identity, Password, Person, Provider, User, UserId, Username,
    };
    use crate::infrastructure::mocks;

    #[test]
    fn next_id() -> Result<(), Error> {
        let repo = InMemUserRepository::new();
        let id1 = repo.next_id()?;
        let id2 = repo.next_id()?;
        let id3 = repo.next_id()?;
        assert!(id1.len() > 10);
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);

        Ok(())
    }

    #[test]
    fn find_by_id() -> Result<(), Error> {
        let repo = InMemUserRepository::new();
        let user = mocks::user1()?;
        let mut changed_user = user.clone();
        changed_user.set_person(Person::new(Fullname::new("Name", "Lastname")?)?)?;

        repo.save(&mut changed_user)?;
        assert_eq!(repo.users.borrow().len(), 1);
        assert!(user.person().is_none());

        let found_user = repo.find_by_id(&user.base().id())?;
        assert_eq!(user.base(), found_user.base());
        assert_eq!(changed_user.base(), found_user.base());

        let changed_user_person = found_user.person().ok_or(Error::internal())?;
        assert_eq!(changed_user_person.fullname().name(), "Name");
        assert_eq!(changed_user_person.fullname().lastname(), "Lastname");

        let _found_user = repo.find_by_username_or_email("username")?;
        let _found_user = repo.find_by_username_or_email("username@email.com")?;
        assert!(repo.find_by_username_or_email("nonexisting").is_err());
        assert!(repo.find_by_username_or_email("username@asd.com").is_err());

        Ok(())
    }
}
