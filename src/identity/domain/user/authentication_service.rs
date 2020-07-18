use std::rc::Rc;

use crate::common::error::Error;
use crate::common::model::Entity;
use crate::identity::domain::token::{Data, Token, TokenService};
use crate::identity::domain::user::{PasswordHasher, User, UserRepository};

pub struct UserDescriptor {
    user: User,
    token: Token,
}

impl UserDescriptor {
    fn user(&self) -> &User {
        &self.user
    }
    fn token(&self) -> &Token {
        &self.token
    }
}

pub trait AuthenticationService {
    fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<UserDescriptor, Error>;
}

pub struct AuthenticationServiceImpl<TUserRepository, TPasswordHasher, TTokenService> {
    user_repository: Rc<TUserRepository>,
    password_hasher: Rc<TPasswordHasher>,
    token_service: Rc<TTokenService>,
}

impl<TUserRepository, TPasswordHasher, TTokenService>
    AuthenticationServiceImpl<TUserRepository, TPasswordHasher, TTokenService>
where
    TUserRepository: UserRepository,
    TPasswordHasher: PasswordHasher,
    TTokenService: TokenService,
{
    pub fn new(
        user_repository: Rc<TUserRepository>,
        password_hasher: Rc<TPasswordHasher>,
        token_service: Rc<TTokenService>,
    ) -> Self {
        AuthenticationServiceImpl {
            user_repository,
            password_hasher,
            token_service,
        }
    }
}

impl<TUserRepository, TPasswordHasher, TTokenService> AuthenticationService
    for AuthenticationServiceImpl<TUserRepository, TPasswordHasher, TTokenService>
where
    TUserRepository: UserRepository,
    TPasswordHasher: PasswordHasher,
    TTokenService: TokenService,
{
    fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<UserDescriptor, Error> {
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

            return Ok(UserDescriptor { user, token });
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::domain::role::Role;
    use crate::identity::domain::token::TokenServiceImpl;
    use crate::identity::domain::user::User;
    use crate::identity::infrastructure::mocks::{FakePasswordHasher, FakeTokenEncoder};
    use crate::identity::infrastructure::persistence::inmem::{
        InMemTokenRepository, InMemUserRepository,
    };

    #[test]
    fn authenticate() -> Result<(), Error> {
        let user_repo = Rc::new(InMemUserRepository::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_serv = Rc::new(TokenServiceImpl::new(
            FakeTokenEncoder::new(),
            InMemTokenRepository::new(),
        ));

        let serv = AuthenticationServiceImpl::new(
            Rc::clone(&user_repo),
            Rc::clone(&password_hasher),
            Rc::clone(&token_serv),
        );

        let mut user = User::new(
            "U002".to_owned(),
            "user1",
            "user@email.com",
            &password_hasher.hash("user123")?,
            &Role::new("user".to_owned(), "User")?,
        )?;
        user_repo.save(&mut user)?;

        let UserDescriptor { user, token } = serv.authenticate("user1", "user123")?;
        assert_eq!(user.id().value(), "U002");
        assert!(token.token().len() > 0);

        let UserDescriptor { user, token } = serv.authenticate("user@email.com", "user123")?;
        assert_eq!(user.id().value(), "U002");
        assert!(token.token().len() > 0);

        assert!(serv.authenticate("user2", "user123").is_err());
        assert!(serv.authenticate("user1", "user124").is_err());
        assert!(serv.authenticate("user@email.com.ar", "user123").is_err());
        assert!(serv.authenticate("user@email.com", "user124").is_err());

        Ok(())
    }
}
