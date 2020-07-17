use std::rc::Rc;

use crate::common::error::Error;
use crate::identity::domain::user::{PasswordHasher, UserID, UserRepository};

pub struct AuthorizationService<TUserRepository, TPasswordHasher> {
    user_repository: Rc<TUserRepository>,
    password_hasher: Rc<TPasswordHasher>,
}

impl<TUserRepository, TPasswordHasher> AuthorizationService<TUserRepository, TPasswordHasher>
where
    TUserRepository: UserRepository,
    TPasswordHasher: PasswordHasher,
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        password_hasher: Rc<TPasswordHasher>,
    ) -> AuthorizationService<TUserRepository, TPasswordHasher> {
        AuthorizationService {
            user_repository,
            password_hasher,
        }
    }

    pub fn available(&self, username: &str, email: &str) -> Result<bool, Error> {
        let mut err = Error::application();
        if self
            .user_repository
            .find_by_username_or_email(username)
            .is_ok()
        {
            err.add_context("username", "not_available");
        }
        if self
            .user_repository
            .find_by_username_or_email(email)
            .is_ok()
        {
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
