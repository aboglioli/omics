use uuid::Uuid;

use crate::domain::user::{User, UserID};
use common::error::Error;
use common::model::Entity;

pub type ValidationCode = String;

pub struct Validation {
    code: ValidationCode,
    user_id: UserID,
    used: bool,
}

impl Validation {
    pub fn new(user: &User) -> Validation {
        let uuid = Uuid::new_v4();
        Validation {
            code: uuid.to_string(),
            user_id: user.id().value(),
            used: false,
        }
    }

    pub fn code(&self) -> &ValidationCode {
        &self.code
    }

    pub fn user_id(&self) -> &UserID {
        &self.user_id
    }

    pub fn used(&self) -> bool {
        self.used
    }

    pub fn validate_user(&mut self, user: &mut User, code: &ValidationCode) -> Result<(), Error> {
        if self.used {
            return Err(Error::application());
        }

        if self.code == *code && user.id().value() == self.user_id {
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

    use crate::domain::role::{Role, RoleID};
    use crate::domain::user::User;
    use crate::infrastructure::mocks;

    #[test]
    fn create() -> Result<(), Error> {
        let user = mocks::user1()?;
        let v = Validation::new(&user);
        assert!(!v.code().is_empty());
        assert_eq!(v.user_id(), &user.id().value());

        Ok(())
    }
}
