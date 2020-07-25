use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::user::{User, UserId};

pub type ValidationCode = String;

pub struct Validation {
    base: AggregateRoot<ValidationCode>,
    user_id: UserId,
    used: bool,
}

impl Validation {
    pub fn new(code: ValidationCode, user_id: UserId) -> Result<Validation, Error> {
        let uuid = Uuid::new_v4();
        Ok(Validation {
            base: AggregateRoot::new(code),
            user_id,
            used: false,
        })
    }

    pub fn base(&self) -> &AggregateRoot<ValidationCode> {
        &self.base
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn used(&self) -> bool {
        self.used
    }

    pub fn validate_user(&mut self, user: &mut User, code: &ValidationCode) -> Result<(), Error> {
        if self.used {
            return Err(Error::application());
        }

        if self.base().id() == *code && user.base().id() == self.user_id {
            user.validate();
            self.used = true;
            return Ok(());
        }

        Err(Error::application())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::role::RoleId;
    use crate::domain::user::User;
    use crate::infrastructure::mocks;

    #[test]
    fn create() -> Result<(), Error> {
        let user = mocks::user1()?;
        let v = Validation::new(ValidationCode::from("cod47"), user.base().id()).unwrap();
        assert!(!v.base().id().is_empty());
        assert_eq!(v.user_id(), &user.base().id());

        Ok(())
    }
}
