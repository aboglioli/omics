use common::error::Error;

use crate::domain::user::UserID;
use crate::domain::validation::{Validation, ValidationCode, ValidationRepository};
use crate::infrastructure::mocks;

pub struct InMemValidationRepository;

impl InMemValidationRepository {
    pub fn new() -> InMemValidationRepository {
        InMemValidationRepository
    }
}

impl ValidationRepository for InMemValidationRepository {
    fn find_by_code(&self, code: &ValidationCode) -> Result<Validation, Error> {
        if code == "valid-code" {
            let user = mocks::user1()?;
            return Validation::new(ValidationCode::from("valid-code"), user.base().id());
        }

        Err(Error::internal())
    }
    fn save(&self, validation: &mut Validation) -> Result<(), Error> {
        if validation.base().id() != "valid-code" {
            return Err(Error::internal());
        }
        Ok(())
    }
}
