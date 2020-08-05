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
    validation: Option<Validation>,
}

impl User {
    pub fn new(id: UserId, identity: Identity, role_id: RoleId) -> Result<User> {
        Ok(User {
            base: AggregateRoot::new(id),
            identity,
            person: None,
            role_id,
            validation: Some(Validation::new()?),
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

    pub fn validation(&self) -> Option<&Validation> {
        self.validation.as_ref()
    }

    pub fn is_validated(&self) -> bool {
        self.validation.is_none()
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
            return Err(Error::new("user", "already_validated"));
        }

        self.validation = match self.validation.take() {
            Some(validation) => {
                if validation.validate(code) {
                    None
                } else {
                    Some(validation)
                }
            }
            None => None,
        };

        if !self.is_validated() {
            return Err(Error::new("validation", "not_validated"));
        }

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

        assert!(!user.is_validated());
        assert!(!user.is_active());
        assert!(user.validation().is_some());

        let code = user.validation().unwrap().code().clone();

        assert!(user.validate(&code).is_ok());

        assert!(user.is_validated());
        assert!(user.is_active());
        assert!(user.validation().is_none());

        assert!(user.validate(&code).is_err());
        assert!(user.validation().is_none());
    }
}
