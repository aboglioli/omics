use std::rc::Rc;

use common::error::Error;
use common::model::Entity;
use crate::domain::token::{Data, Token, TokenService};
use crate::domain::user::{PasswordHasher, User, UserID, UserRepository};

pub trait AuthService {
    fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Token, Error>;
    fn authorize(&self, token: Token) -> Result<User, Error>;
    fn available(&self, username: &str, email: &str) -> Result<bool, Error>;
    fn change_password(
        &self,
        user_id: UserID,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error>;
    fn generate_password(&self, plain_pasword: &str) -> Result<String, Error>;
}

pub struct AuthServiceImpl<TUserRepository, TTokenService, TPasswordHasher> {
    user_repository: Rc<TUserRepository>,
    token_service: Rc<TTokenService>,
    password_hasher: Rc<TPasswordHasher>,
}

impl<TUserRepository, TTokenService, TPasswordHasher>
    AuthServiceImpl<TUserRepository, TTokenService, TPasswordHasher>
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        token_service: Rc<TTokenService>,
        password_hasher: Rc<TPasswordHasher>,
    ) -> Self {
        AuthServiceImpl {
            user_repository,
            token_service,
            password_hasher,
        }
    }
}

impl<TUserRepository, TTokenService, TPasswordHasher> AuthService
    for AuthServiceImpl<TUserRepository, TTokenService, TPasswordHasher>
where
    TUserRepository: UserRepository,
    TTokenService: TokenService,
    TPasswordHasher: PasswordHasher,
{
    fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Token, Error> {
        let user = self
            .user_repository
            .find_by_username_or_email(username_or_email)?;
        if self
            .password_hasher
            .compare(user.password().value(), password)
        {
            let mut data = Data::new();
            data.add("user_id", &user.id().value());
            let token = self.token_service.create(data)?;

            return Ok(token);
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }

    fn authorize(&self, token: Token) -> Result<User, Error> {
        let data = self.token_service.validate(token)?;
        if let Some(user_id) = data.get("user_id") {
            let user = self.user_repository.find_by_id(user_id.to_owned())?;
            return Ok(user);
        }
        Err(Error::application())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::role::Role;
    use crate::domain::token::TokenServiceImpl;
    use crate::domain::user::User;
    use crate::infrastructure::mocks::{FakePasswordHasher, FakeTokenEncoder};
    use crate::infrastructure::persistence::inmem::{
        InMemTokenRepository, InMemUserRepository,
    };

    #[test]
    fn authenticate() -> Result<(), Error> {
        let user_repo = Rc::new(InMemUserRepository::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_enc = Rc::new(FakeTokenEncoder::new());
        let token_repo = Rc::new(InMemTokenRepository::new());
        let token_serv = Rc::new(TokenServiceImpl::new(
            Rc::clone(&token_enc),
            Rc::clone(&token_repo),
        ));

        let serv = AuthServiceImpl::new(
            Rc::clone(&user_repo),
            Rc::clone(&token_serv),
            Rc::clone(&password_hasher),
        );

        let mut user = User::new(
            "U002".to_owned(),
            "user1",
            "user@email.com",
            &password_hasher.hash("user123")?,
            &Role::new("user".to_owned(), "User")?,
        )?;
        user_repo.save(&mut user)?;

        let token = serv.authenticate("user1", "user123")?;
        assert!(token.token().len() > 0);

        let token = serv.authenticate("user@email.com", "user123")?;
        assert!(token.token().len() > 0);

        assert!(serv.authenticate("user2", "user123").is_err());
        assert!(serv.authenticate("user1", "user124").is_err());
        assert!(serv.authenticate("user@email.com.ar", "user123").is_err());
        assert!(serv.authenticate("user@email.com", "user124").is_err());

        Ok(())
    }
}
