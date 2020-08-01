use async_trait::async_trait;

use common::error::Error;
use common::result::Result;

use crate::domain::validation::{Validation, ValidationCode, ValidationRepository};
use crate::infrastructure::mocks;

pub struct InMemValidationRepository;

impl InMemValidationRepository {
    pub fn new() -> InMemValidationRepository {
        InMemValidationRepository
    }
}

#[async_trait]
impl ValidationRepository for InMemValidationRepository {
    async fn find_by_code(&self, code: &ValidationCode) -> Result<Validation> {
        if code == "valid-code" {
            let user = mocks::user1()?;
            return Validation::new(ValidationCode::from("valid-code"), user.base().id());
        }

        Err(Error::internal())
    }
    async fn save(&self, validation: &mut Validation) -> Result<()> {
        if validation.base().id() != "valid-code" {
            return Err(Error::internal());
        }
        Ok(())
    }
}
