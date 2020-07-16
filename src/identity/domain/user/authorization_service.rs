use crate::common::error::Error;
use crate::identity::domain::user::{PasswordHasher, UserID, UserRepository};

pub struct AuthorizationService<'a, UR, PH> {
    user_repository: &'a UR,
    password_hasher: &'a PH,
}

impl<'a, UR, PH> AuthorizationService<'_, UR, PH>
where
    UR: UserRepository,
    PH: PasswordHasher,
{
    pub fn new<'b>(
        user_repository: &'b UR,
        password_hasher: &'b PH,
    ) -> AuthorizationService<'b, UR, PH> {
        AuthorizationService {
            user_repository,
            password_hasher,
        }
    }

    pub fn available(&self, username: &str, email: &str) -> Result<bool, Error> {
        let mut err = Error::application();
        if let Ok(_) = self.user_repository.find_by_username_or_email(username) {
            err.add_context("username", "not_available");
        }
        if let Ok(_) = self.user_repository.find_by_username_or_email(email) {
            err.add_context("email", "not_available");
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(true)
    }

    pub fn change_password(
        &self,
        user_id: UserID,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        let mut user = self.user_repository.find_by_id(user_id)?;
        if !self
            .password_hasher
            .compare(user.password().value(), old_password)
        {
            return Err(Error::application().set_code("invalid_password").clone());
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        user.set_password(&hashed_password)?;

        self.user_repository.save(&mut user)?;

        Ok(())
    }

    pub fn generate_password(&self, plain_pasword: &str) -> Result<String, Error> {
        self.password_hasher.hash(plain_pasword)
    }
}
