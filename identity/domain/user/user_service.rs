use common::error::Error;
use common::result::Result;

use crate::domain::user::{Email, Password, PasswordHasher, UserId, UserRepository, Username};

pub struct UserService<'a, URepo, PHasher> {
    user_repo: &'a URepo,

    password_hasher: &'a PHasher,
}

impl<'a, URepo, PHasher> UserService<'a, URepo, PHasher>
where
    URepo: UserRepository,
    PHasher: PasswordHasher,
{
    pub fn new(user_repo: &'a URepo, password_hasher: &'a PHasher) -> Self {
        UserService {
            user_repo,
            password_hasher,
        }
    }

    pub async fn available(&self, username: &str, email: &str) -> Result<bool> {
        let mut err = Error::new("identity", "invalid");
        if self
            .user_repo
            .find_by_username(&Username::new(username)?)
            .await
            .is_ok()
        {
            err.add_context("username", "not_available");
        }
        if self
            .user_repo
            .find_by_email(&Email::new(email)?)
            .await
            .is_ok()
        {
            err.add_context("email", "not_available");
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(true)
    }

    pub async fn change_password(
        &self,
        user_id: &UserId,
        old_password: &str,
        new_password: &str,
    ) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(Error::new("password", "unavailable")),
        };

        if !self.password_hasher.compare(user_password, old_password) {
            return Err(Error::new("password", "invalid"));
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        let password = Password::new(&hashed_password)?;
        user.set_password(password)?;

        self.user_repo.save(&mut user).await?;

        Ok(())
    }

    pub fn generate_password(&self, plain_pasword: &str) -> Result<String> {
        self.password_hasher.hash(plain_pasword)
    }
}
