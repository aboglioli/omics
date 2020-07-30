use common::error::Error;

use crate::domain::user::{UserId, UserRepository};
use crate::domain::validation::{ValidationCode, ValidationRepository};

pub struct Validate<'a, URepo, VRepo> {
    user_repo: &'a URepo,
    validation_repo: &'a VRepo,
}

impl<'a, URepo, VRepo> Validate<'a, URepo, VRepo>
where
    URepo: UserRepository,
    VRepo: ValidationRepository,
{
    pub fn new(user_repo: &'a URepo, validation_repo: &'a VRepo) -> Self {
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
