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

        let hashed_password = self.generate_password(new_password)?;

        let password = Password::new(&hashed_password)?;
        user.set_password(password)?;

        self.user_repo.save(&mut user).await?;

        Ok(())
    }

    pub fn generate_password(&self, plain_password: &str) -> Result<String> {
        // TODO: improve validation
        if plain_password.len() < 8 {
            return Err(Error::new("password", "too_short"));
        }

        self.password_hasher.hash(plain_password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn check_availability() {
        let c = mocks::container();
        let uc = UserService::new(c.user_repo(), c.password_hasher());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .available(
                user.identity().username().value(),
                user.identity().email().value()
            )
            .await
            .is_err());
        assert!(uc
            .available(user.identity().username().value(), "new@email.com")
            .await
            .is_err());
        assert!(uc
            .available("new-user", user.identity().email().value())
            .await
            .is_err());
        assert!(uc.available("new-user", "new@email.com").await.is_ok());
    }

    #[tokio::test]
    async fn change_password() {
        let c = mocks::container();
        let uc = UserService::new(c.user_repo(), c.password_hasher());

        let mut user = mocks::user1();
        c.user_repo().save(&mut user).await.unwrap();

        assert!(uc
            .change_password(
                &UserId::new("#invalid-id").unwrap(),
                "P@asswd!",
                "new-password"
            )
            .await
            .is_err());
        assert!(uc
            .change_password(&user.base().id(), "P@asswd!", "123")
            .await
            .is_err());
        assert!(uc
            .change_password(&user.base().id(), "invalid-password", "New_P@asswd!")
            .await
            .is_err());
        assert!(uc
            .change_password(&user.base().id(), "P@asswd!", "New_P@asswd!")
            .await
            .is_ok());
    }

    #[test]
    fn generate_password() {
        let c = mocks::container();
        let uc = UserService::new(c.user_repo(), c.password_hasher());

        assert!(uc.generate_password("123").is_err());
        assert!(uc.generate_password("abc123").is_err());
        assert!(uc.generate_password("P@asswd!").is_ok());
    }
}
