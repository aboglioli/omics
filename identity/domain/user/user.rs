use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::role::RoleID;
use crate::domain::user::{Email, Identity, Password, Person, Provider, Username};

// User
pub type UserID = String;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserID>,
    identity: Identity,
    person: Option<Person>,
    role_id: RoleID,
    validated: bool,
}

impl User {
    pub fn new(id: UserID, identity: Identity, role_id: RoleID) -> Result<User, Error> {
        Ok(User {
            base: AggregateRoot::new(id),
            identity,
            person: None,
            role_id,
            validated: false,
        })
    }

    pub fn base(&self) -> &AggregateRoot<UserID> {
        &self.base
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn person(&self) -> Option<&Person> {
        self.person.as_ref()
    }

    pub fn role_id(&self) -> &RoleID {
        &self.role_id
    }

    pub fn is_validated(&self) -> bool {
        self.validated
    }

    pub fn is_active(&self) -> bool {
        self.base.deleted_at().is_none() && self.validated
    }

    pub fn set_password(&mut self, password: Password) -> Result<(), Error> {
        self.identity.set_password(password)?;
        Ok(())
    }

    pub fn set_person(&mut self, person: Person) -> Result<(), Error> {
        self.person = Some(person);
        Ok(())
    }

    pub fn set_role(&mut self, role_id: RoleID) {
        self.role_id = role_id
    }

    pub fn validate(&mut self) {
        self.validated = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() -> Result<(), Error> {
        let user = User::new(
            UserID::from("user123"),
            Identity::new(
                Provider::Local,
                Username::new("user1")?,
                Email::new("email@user.com")?,
                Some(Password::new(&format!("{:X>50}", "2"))?),
            )?,
            RoleID::from("user"),
        )?;
        assert_eq!(user.base().id(), "user123");
        assert_eq!(user.identity().username().value(), "user1");
        assert_eq!(user.identity().email().value(), "email@user.com");

        Ok(())
    }
}
