use std::rc::Rc;

use crate::domain::token::{Data, Token, TokenService};
use crate::domain::user::{Password, PasswordHasher, User, UserID, UserRepository};
use common::error::Error;
use common::model::Entity;

pub struct AuthService {
    user_repository: Rc<dyn UserRepository>,
    token_service: Rc<TokenService>,
    password_hasher: Rc<dyn PasswordHasher>,
}

impl AuthService {
    pub fn new(
        user_repository: Rc<dyn UserRepository>,
        token_service: Rc<TokenService>,
        password_hasher: Rc<dyn PasswordHasher>,
    ) -> Self {
        AuthService {
            user_repository,
            token_service,
            password_hasher,
        }
    }

    pub fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Token, Error> {
        let user = self
            .user_repository
            .find_by_username_or_email(username_or_email)?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(Error::application()),
        };

        if self.password_hasher.compare(user_password, password) {
            let mut data = Data::new();
            data.add("user_id", &user.id().value());
            let token = self.token_service.create(data)?;

            return Ok(token);
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }

    pub fn authorize(&self, token: &Token) -> Result<User, Error> {
        let data = self.token_service.validate(token)?;
        if let Some(user_id) = data.get("user_id") {
            let user = self.user_repository.find_by_id(user_id)?;
            return Ok(user);
        }
        Err(Error::application())
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
        user_id: &UserID,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        let mut user = self.user_repository.find_by_id(user_id)?;

        let user_password = match user.identity().password() {
            Some(password) => password.value(),
            None => return Err(Error::application()),
        };

        if !self.password_hasher.compare(user_password, old_password) {
            return Err(Error::application().set_code("invalid_password").build());
        }

        let hashed_password = self.password_hasher.hash(new_password)?;

        let password = Password::new(&hashed_password)?;
        user.set_password(password)?;

        self.user_repository.save(&mut user)?;

        Ok(())
    }

    pub fn generate_password(&self, plain_pasword: &str) -> Result<String, Error> {
        self.password_hasher.hash(plain_pasword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::role::*;
    use crate::domain::token::*;
    use crate::domain::user::*;
    use crate::infrastructure::mocks::{self, *};
    use crate::infrastructure::persistence::inmem::*;

    #[test]
    fn authenticate() -> Result<(), Error> {
        let user_repo = Rc::new(InMemUserRepository::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_enc = Rc::new(FakeTokenEncoder::new());
        let token_repo = Rc::new(InMemTokenRepository::new());
        let token_serv = Rc::new(TokenService::new(
            Rc::clone(&token_enc) as Rc<dyn TokenEncoder>,
            Rc::clone(&token_repo) as Rc<dyn TokenRepository>,
        ));

        let serv = AuthService::new(
            Rc::clone(&user_repo) as Rc<dyn UserRepository>,
            Rc::clone(&token_serv),
            Rc::clone(&password_hasher) as Rc<dyn PasswordHasher>,
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
