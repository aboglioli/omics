use std::sync::Arc;

use common::error::Error;

use crate::domain::user::{
    AuthService, Email, Fullname, Identity, Password, Person, Provider, User, UserId,
    UserRepository, UserUpdated, Username,
};
use crate::domain::validation::{ValidationCode, ValidationRepository};

pub struct Validate {
    user_repo: Arc<dyn UserRepository>,
    validation_repo: Arc<dyn ValidationRepository>,
}

impl Validate {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        validation_repo: Arc<dyn ValidationRepository>,
    ) -> Self {
        Validate {
            user_repo,
            validation_repo,
        }
    }

    pub fn exec(&self, user_id: &UserId, validation_code: &ValidationCode) -> Result<(), Error> {
        let mut user = self.user_repo.find_by_id(user_id)?;
        let mut validation = self.validation_repo.find_by_code(validation_code)?;
        validation.validate_user(&mut user, validation_code)?;

        self.user_repo.save(&mut user)?;
        self.validation_repo.save(&mut validation)?;

        Ok(())
    }
}
