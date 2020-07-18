use std::rc::Rc;

use crate::common::error::Error;
use crate::identity::domain::user::{PasswordHasher, UserID, UserRepository};

pub trait AuthorizationService {
    fn available(&self, username: &str, email: &str) -> Result<bool, Error>;
    fn change_password(
        &self,
        user_id: UserID,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error>;
    fn generate_password(&self, plain_pasword: &str) -> Result<String, Error>;
}

pub struct AuthorizationServiceImpl<TUserRepository, TPasswordHasher> {
    user_repository: Rc<TUserRepository>,
    password_hasher: Rc<TPasswordHasher>,
}

impl<TUserRepository, TPasswordHasher> AuthorizationServiceImpl<TUserRepository, TPasswordHasher>
where
    TUserRepository: UserRepository,
    TPasswordHasher: PasswordHasher,
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        password_hasher: Rc<TPasswordHasher>,
    ) -> AuthorizationServiceImpl<TUserRepository, TPasswordHasher> {
        AuthorizationServiceImpl {
            user_repository,
            password_hasher,
        }
    }
}

impl<TUserRepository, TPasswordHasher> AuthorizationService
    for AuthorizationServiceImpl<TUserRepository, TPasswordHasher>
where
    TUserRepository: UserRepository,
    TPasswordHasher: PasswordHasher,
{
    fn available(&self, username: &str, email: &str) -> Result<bool, Error> {
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

    fn change_password(
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
            return Err(Error::application().set_code("invalid_password").build());
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        user.set_password(&hashed_password)?;

        self.user_repository.save(&mut user)?;

        Ok(())
    }

    fn generate_password(&self, plain_pasword: &str) -> Result<String, Error> {
        self.password_hasher.hash(plain_pasword)
    }
}
