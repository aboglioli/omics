use std::sync::Arc;

use common::error::Error;

use crate::domain::user::{UserId, UserRepository};
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

    pub async fn exec(
        &self,
        user_id: &UserId,
        validation_code: &ValidationCode,
    ) -> Result<(), Error> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        let mut validation = self.validation_repo.find_by_code(validation_code).await?;
        validation.validate_user(&mut user, validation_code)?;

        self.user_repo.save(&mut user).await?;
        self.validation_repo.save(&mut validation).await?;

        Ok(())
    }
}
