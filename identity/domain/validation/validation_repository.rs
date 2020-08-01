use async_trait::async_trait;

use common::result::Result;

use crate::domain::validation::{Validation, ValidationCode};

#[async_trait]
pub trait ValidationRepository {
    async fn find_by_code(&self, code: &ValidationCode) -> Result<Validation>;
    async fn save(&self, validation: &mut Validation) -> Result<()>;
}
