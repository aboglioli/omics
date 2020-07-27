use std::sync::Arc;

use common::error::Error;

use crate::domain::token::{Data, Token, TokenService};
use crate::domain::user::{
    Email, Password, PasswordHasher, User, UserId, UserRepository, Username,
};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    token_serv: Arc<TokenService>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_serv: Arc<TokenService>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        AuthService {
            user_repo,
            token_serv,
            password_hasher,
        }
    }

    pub fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Token, Error> {
        let mut err = Error::pair("credentials", "invalid");

        let user = self
            .user_repo
            .find_by_username(&Username::new(username_or_email)?)
            .or(self
                .user_repo
                .find_by_email(&Email::new(username_or_email)?))?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(err),
        };

        if self.password_hasher.compare(user_password, password) {
            let mut data = Data::new();
            data.add("user_id", &user.base().id());
            let token = match self.token_serv.create(data) {
                Ok(token) => token,
                Err(e) => return Err(err.wrap(e).build()),
            };

            return Ok(token);
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }

    pub fn authorize(&self, token: &Token) -> Result<User, Error> {
        let data = self.token_serv.validate(token)?;
        if let Some(user_id) = data.get("user_id") {
            let user = self.user_repo.find_by_id(user_id)?;
            return Ok(user);
        }
        Err(Error::application())
    }

    pub fn available(&self, username: &str, email: &str) -> Result<bool, Error> {
        let mut err = Error::application();
        if self
            .user_repo
            .find_by_username(&Username::new(username)?)
            .is_ok()
        {
            err.add_context("username", "not_available");
        }
        if self.user_repo.find_by_email(&Email::new(email)?).is_ok() {
            err.add_context("email", "not_available");
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(true)
    }

    pub fn change_password(
        &self,
        user_id: &UserId,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        let mut user = self.user_repo.find_by_id(user_id)?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(Error::pair("password", "unavailable")),
        };

        if !self.password_hasher.compare(user_password, old_password) {
            return Err(Error::application().set_code("invalid_password").build());
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        let password = Password::new(&hashed_password)?;
        user.set_password(password)?;

        self.user_repo.save(&mut user)?;

        Ok(())
    }

    pub fn generate_password(&self, plain_pasword: &str) -> Result<String, Error> {
        self.password_hasher.hash(plain_pasword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::token::*;
    use crate::domain::user::*;
    use crate::infrastructure::mocks::{self, *};
    use crate::infrastructure::persistence::inmem::*;

    #[test]
    fn authenticate() -> Result<(), Error> {
        let user_repo = Arc::new(InMemUserRepository::new());
        let password_hasher = Arc::new(FakePasswordHasher::new());
        let token_enc = Arc::new(FakeTokenEncoder::new());
        let token_repo = Arc::new(InMemTokenRepository::new());
        let token_serv = Arc::new(TokenService::new(
            Arc::clone(&token_enc) as Arc<dyn TokenEncoder>,
            Arc::clone(&token_repo) as Arc<dyn TokenRepository>,
        ));

        let serv = AuthService::new(
            Arc::clone(&user_repo) as Arc<dyn UserRepository>,
            Arc::clone(&token_serv),
            Arc::clone(&password_hasher) as Arc<dyn PasswordHasher>,
        );

        let mut user = mocks::user1()?;
        user_repo.save(&mut user)?;

        let res = serv.authenticate("username", "P@asswd!");
        assert!(res.is_ok());
        assert!(!res.unwrap().token().is_empty());

        let res = serv.authenticate("username@email.com", "P@asswd!");
        assert!(res.is_ok());
        assert!(!res.unwrap().token().is_empty());

        assert!(serv.authenticate("user2", "user123").is_err());
        assert!(serv.authenticate("user1", "user124").is_err());
        assert!(serv.authenticate("user@email.com.ar", "user123").is_err());
        assert!(serv.authenticate("user@email.com", "user124").is_err());

        Ok(())
    }
}
