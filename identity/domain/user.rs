mod authentication_service;
mod authorization_service;
mod email;
mod fullname;
mod identity;
mod password;
mod password_hasher;
mod person;
mod provider;
mod user_repository;
mod user_service;
mod username;
mod validation;
pub use self::identity::*;
pub use authentication_service::*;
pub use authorization_service::*;
pub use email::*;
pub use fullname::*;
pub use password::*;
pub use password_hasher::*;
pub use person::*;
pub use provider::*;
pub use user_repository::*;
pub use user_service::*;
pub use username::*;
pub use validation::*;

use common::error::Error;
use common::model::{AggregateRoot, StringId};
use common::result::Result;
use shared::domain::event::UserEvent;

use crate::domain::role::Role;

pub type UserId = StringId;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId, UserEvent>,
    identity: Identity,
    person: Option<Person>,
    role: Role,
    validation: Option<Validation>,
}

impl User {
    pub fn new(id: UserId, identity: Identity, role: Role) -> Result<Self> {
        let mut user = User {
            base: AggregateRoot::new(id),
            identity,
            person: None,
            role,
            validation: Some(Validation::new()?),
        };

        user.base.record_event(UserEvent::Registered {
            id: user.base().id().value().to_owned(),
            username: user.identity().username().value().to_owned(),
            email: user.identity().username().value().to_owned(),
        });

        Ok(user)
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

    pub fn role(&self) -> &Role {
        &self.role
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

        self.base.record_event(UserEvent::Updated {
            id: self.base().id().value().to_owned(),
            name: self.person().unwrap().fullname().name().to_owned(),
            lastname: self.person().unwrap().fullname().lastname().to_owned(),
        });

        Ok(())
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
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
            return Err(Error::new("user", "not_validated"));
        }

        self.base.record_event(UserEvent::Validated {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn login(&mut self) -> Result<()> {
        self.base.record_event(UserEvent::LoggedIn {
            id: self.base().id().value().to_owned(),
        });

        Ok(())
    }

    pub fn recover_password(&mut self, password: Password, temp_password: &str) -> Result<()> {
        self.identity.set_password(password)?;
        self.base
            .record_event(UserEvent::PasswordRecoveryRequested {
                id: self.base().id().value().to_owned(),
                temp_password: temp_password.to_owned(),
                email: self.identity().email().value().to_owned(),
            });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::role::RoleId;
    use crate::domain::user::{Email, Provider, Username};

    #[test]
    fn create() {
        let user = User::new(
            UserId::new("user123").unwrap(),
            Identity::new(
                Provider::Local,
                Username::new("user1").unwrap(),
                Email::new("email@user.com").unwrap(),
                Some(Password::new(&format!("{:X>50}", "2")).unwrap()),
            )
            .unwrap(),
            Role::new(RoleId::new("user").unwrap(), "User").unwrap(),
        )
        .unwrap();
        assert_eq!(user.base().id().value(), "user123");
        assert_eq!(user.identity().username().value(), "user1");
        assert_eq!(user.identity().email().value(), "email@user.com");
    }

    #[test]
    fn validate() {
        let mut user = User::new(
            UserId::new("user123").unwrap(),
            Identity::new(
                Provider::Local,
                Username::new("user1").unwrap(),
                Email::new("email@user.com").unwrap(),
                Some(Password::new(&format!("{:X>50}", "2")).unwrap()),
            )
            .unwrap(),
            Role::new(RoleId::new("user").unwrap(), "User").unwrap(),
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
