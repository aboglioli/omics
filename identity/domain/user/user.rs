use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::role::RoleId;
use crate::domain::user::{Identity, Password, Person, UserEvent, Validation, ValidationCode};

// User
pub type UserId = String;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId, UserEvent>,
    identity: Identity,
    person: Option<Person>,
    role_id: RoleId,
    validation: Validation,
}

impl User {
    pub fn new(id: UserId, identity: Identity, role_id: RoleId) -> Result<User> {
        Ok(User {
            base: AggregateRoot::new(id),
            identity,
            person: None,
            role_id,
            validation: Validation::new()?,
        })
    }

    pub fn base(&self) -> &AggregateRoot<UserId, UserEvent> {
        &self.base
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn person(&self) -> Option<&Person> {
        self.person.as_ref()
    }

    pub fn role_id(&self) -> &RoleId {
        &self.role_id
    }

    pub fn validation(&self) -> &Validation {
        &self.validation
    }

    pub fn is_validated(&self) -> bool {
        self.validation.validated()
    }

    pub fn is_active(&self) -> bool {
        self.base.deleted_at().is_none() && self.is_validated()
    }

    pub fn set_password(&mut self, password: Password) -> Result<()> {
        self.identity.set_password(password)?;
        Ok(())
    }

    pub fn set_person(&mut self, person: Person) -> Result<()> {
        self.person = Some(person);
        Ok(())
    }

    pub fn set_role(&mut self, role_id: RoleId) {
        self.role_id = role_id
    }

    pub fn validate(&mut self, code: &ValidationCode) -> Result<()> {
        if self.is_validated() {
            return Err(Error::pair("user", "already_validated"));
        }

        self.validation = self.validation.validate(code)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::user::{Email, Provider, Username};

    #[test]
    fn create() {
        let user = User::new(
            UserId::from("user123"),
            Identity::new(
                Provider::Local,
                Username::new("user1").unwrap(),
                Email::new("email@user.com").unwrap(),
                Some(Password::new(&format!("{:X>50}", "2")).unwrap()),
            )
            .unwrap(),
            RoleId::from("user"),
        )
        .unwrap();
        assert_eq!(user.base().id(), "user123");
        assert_eq!(user.identity().username().value(), "user1");
        assert_eq!(user.identity().email().value(), "email@user.com");
    }

    #[test]
    fn validate() {
        let mut user = User::new(
            UserId::from("user123"),
            Identity::new(
                Provider::Local,
                Username::new("user1").unwrap(),
                Email::new("email@user.com").unwrap(),
                Some(Password::new(&format!("{:X>50}", "2")).unwrap()),
            )
            .unwrap(),
            RoleId::from("user"),
        )
        .unwrap();

        assert_eq!(user.is_validated(), false);
        assert_eq!(user.is_active(), false);

        let code = user.validation().code().clone();

        assert!(user.validate(&code).is_ok());

        assert_eq!(user.is_validated(), true);
        assert_eq!(user.is_active(), true);

        assert!(user.validate(&code).is_err());
    }
}
