use async_trait::async_trait;

use common::error::Error;

use crate::domain::validation::{Validation, ValidationCode};

#[async_trait]
pub trait ValidationRepository {
    async fn find_by_code(&self, code: &ValidationCode) -> Result<Validation, Error>;
    async fn save(&self, validation: &mut Validation) -> Result<(), Error>;
}
