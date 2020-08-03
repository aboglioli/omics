use common::result::Result;

use crate::domain::user::{UserId, UserRepository, ValidationCode};

pub struct Validate<'a, URepo> {
    user_repo: &'a URepo,
}

impl<'a, URepo> Validate<'a, URepo>
where
    URepo: UserRepository,
{
    pub fn new(user_repo: &'a URepo) -> Self {
        Validate { user_repo }
    }

    pub async fn exec(&self, user_id: &UserId, validation_code: &ValidationCode) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        user.validate(validation_code)?;

        self.user_repo.save(&mut user).await?;

        Ok(())
    }
}
