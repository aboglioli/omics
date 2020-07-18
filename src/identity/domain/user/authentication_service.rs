use std::rc::Rc;

use crate::common::error::Error;
use crate::common::model::Entity;
use crate::identity::domain::token::{Data, Token, TokenService};
use crate::identity::domain::user::{PasswordHasher, User, UserRepository};

pub trait AuthenticationService {
    fn authenticate(&self, username_or_email: &str, password: &str)
        -> Result<(User, Token), Error>;
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
    ) -> AuthenticationServiceImpl<TUserRepository, TPasswordHasher, TTokenService> {
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
    ) -> Result<(User, Token), Error> {
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

            return Ok((user, token));
        }
        Err(Error::application().set_code("invalid_credentials").build())
    }
}
