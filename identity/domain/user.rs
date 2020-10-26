mod authentication_service;
mod authorization_service;
mod biography;
mod birthdate;
mod email;
mod fullname;
mod gender;
mod identity;
mod image;
mod password;
mod password_hasher;
mod person;
mod provider;
mod repository;
mod service;
mod username;
mod validation;
pub use self::identity::*;
pub use authentication_service::*;
pub use authorization_service::*;
pub use biography::*;
pub use birthdate::*;
pub use email::*;
pub use fullname::*;
pub use gender::*;
pub use image::*;
pub use password::*;
pub use password_hasher::*;
pub use person::*;
pub use provider::*;
pub use repository::*;
pub use service::*;
pub use username::*;
pub use validation::*;

use common::error::Error;
use common::model::{AggregateRoot, Events, StringId};
use common::result::Result;
use shared::event::UserEvent;

use crate::domain::role::RoleId;
use crate::domain::token::Token;

pub type UserId = StringId;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId>,
    events: Events<UserEvent>,
    identity: Identity,
    person: Option<Person>,
    role_id: RoleId,
    validation: Option<Validation>,
    payment_email: Option<Email>,
}

impl User {
    pub fn new(id: UserId, identity: Identity, role_id: RoleId) -> Result<Self> {
        let mut user = User {
            base: AggregateRoot::new(id),
            events: Events::new(),
            identity,
            person: None,
            role_id,
            validation: Some(Validation::new()),
            payment_email: None,
        };

        user.events.record_event(UserEvent::Registered {
            id: user.base().id().to_string(),
            username: user.identity().username().to_string(),
            email: user.identity().email().to_string(),
            validation_code: user.validation().unwrap().code().to_string(),
        });

        Ok(user)
    }

    pub fn build(
        base: AggregateRoot<UserId>,
        identity: Identity,
        person: Option<Person>,
        role_id: RoleId,
        validation: Option<Validation>,
        payment_email: Option<Email>,
    ) -> Self {
        User {
            base,
            events: Events::new(),
            identity,
            person,
            role_id,
            validation,
            payment_email,
        }
    }

    pub fn base(&self) -> &AggregateRoot<UserId> {
        &self.base
    }

    pub fn events(&self) -> &Events<UserEvent> {
        &self.events
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

    pub fn payment_email(&self) -> Option<&Email> {
        self.payment_email.as_ref()
    }

    pub fn is_validated(&self) -> bool {
        self.validation.is_none()
    }

    pub fn is_active(&self) -> bool {
        self.base.deleted_at().is_none() && self.is_validated()
    }

    pub fn is_admin(&self) -> bool {
        self.role_id.value() == "admin"
    }

    pub fn is_content_manager(&self) -> bool {
        self.role_id.value() == "admin" || self.role_id.value() == "content-manager"
    }

    pub fn set_password(&mut self, password: Password) -> Result<()> {
        self.identity.set_password(password)?;
        self.base.update();
        Ok(())
    }

    pub fn set_person(&mut self, person: Person) -> Result<()> {
        self.person = Some(person);
        self.base.update();

        self.events.record_event(UserEvent::Updated {
            id: self.base().id().to_string(),
            name: self.person().unwrap().fullname().name().to_string(),
            lastname: self.person().unwrap().fullname().lastname().to_string(),
            birthdate: self
                .person()
                .unwrap()
                .birthdate()
                .map(|birthdate| birthdate.date().to_rfc3339()),
            gender: self
                .person()
                .unwrap()
                .gender()
                .map(|gender| gender.to_string()),
            biography: self
                .person()
                .unwrap()
                .biography()
                .map(|biography| biography.to_string()),
            profile_image: self
                .person()
                .unwrap()
                .profile_image()
                .map(|profile_image| profile_image.to_string()),
        });

        Ok(())
    }

    pub fn change_role(&mut self, role_id: RoleId) -> Result<()> {
        self.role_id = role_id;
        self.base.update();

        self.events.record_event(UserEvent::RoleChanged {
            id: self.base().id().to_string(),
            role_id: self.role_id().to_string(),
        });

        Ok(())
    }

    pub fn validate(&mut self, val: &Validation) -> Result<()> {
        if self.is_validated() {
            return Err(Error::new("user", "already_validated"));
        }

        self.validation = match self.validation.take() {
            Some(validation) => {
                if &validation == val {
                    None
                } else {
                    Some(validation)
                }
            }
            None => None,
        };

        if !self.is_validated() {
            return Err(Error::new("user", "invalid_code"));
        }

        self.base.update();

        self.events.record_event(UserEvent::Validated {
            id: self.base().id().to_string(),
        });

        Ok(())
    }

    pub fn login(&mut self, token: &Token) -> Result<()> {
        if !self.is_validated() {
            return Err(Error::new("user", "not_validated"));
        }

        if !self.is_active() {
            return Err(Error::new("user", "not_active"));
        }

        self.events.record_event(UserEvent::LoggedIn {
            id: self.base().id().to_string(),
            auth_token: token.to_string(),
        });

        Ok(())
    }

    pub fn recover_password(&mut self, password: Password, temp_password: String) -> Result<()> {
        self.identity.set_password(password)?;
        self.events
            .record_event(UserEvent::PasswordRecoveryRequested {
                id: self.base().id().to_string(),
                temp_password,
                email: self.identity().email().to_string(),
            });

        Ok(())
    }

    pub fn set_payment_email(&mut self, payment_email: Email) -> Result<()> {
        self.payment_email = Some(payment_email);
        self.base.update();

        self.events.record_event(UserEvent::PaymentEmailChanged {
            id: self.base().id().to_string(),
            payment_email: self.payment_email().unwrap().to_string(),
        });

        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        if !self.is_active() {
            return Err(Error::new("user", "not_active"));
        }

        self.base.delete();

        self.events.record_event(UserEvent::Deleted {
            id: self.base().id().to_string(),
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
            RoleId::new("user").unwrap(),
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
            RoleId::new("user").unwrap(),
        )
        .unwrap();

        assert!(!user.is_validated());
        assert!(!user.is_active());
        assert!(user.validation().is_some());

        let code = user.validation().unwrap().clone();
        assert!(user.validate(&code).is_ok());

        assert!(user.is_validated());
        assert!(user.is_active());
        assert!(user.validation().is_none());

        assert!(user.validate(&code).is_err());
        assert!(user.validation().is_none());
    }

    #[test]
    fn delete() {
        let mut user = User::new(
            UserId::new("user123").unwrap(),
            Identity::new(
                Provider::Local,
                Username::new("user1").unwrap(),
                Email::new("email@user.com").unwrap(),
                Some(Password::new(&format!("{:X>50}", "2")).unwrap()),
            )
            .unwrap(),
            RoleId::new("user").unwrap(),
        )
        .unwrap();

        assert!(user.delete().is_err());

        let code = user.validation().unwrap().clone();
        assert!(user.validate(&code).is_ok());

        assert!(user.delete().is_ok());
        assert!(user.delete().is_err());
    }
}
