use std::sync::Arc;

use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::domain::role::RoleRepository;
use crate::domain::token::{TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, UserRepository, UserService,
};

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    role_repo: Arc<dyn RoleRepository>,
    token_repo: Arc<dyn TokenRepository>,
    user_repo: Arc<dyn UserRepository>,

    password_hasher: Arc<dyn PasswordHasher>,
    token_enc: Arc<dyn TokenEncoder>,

    token_serv: Arc<TokenService>,
    user_serv: Arc<UserService>,
    authentication_serv: Arc<AuthenticationService>,
    authorization_serv: Arc<AuthorizationService>,
}

impl<EPub> Container<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,

        role_repo: Arc<dyn RoleRepository>,
        token_repo: Arc<dyn TokenRepository>,
        user_repo: Arc<dyn UserRepository>,

        password_hasher: Arc<dyn PasswordHasher>,
        token_enc: Arc<dyn TokenEncoder>,
    ) -> Self {
        let token_serv = Arc::new(TokenService::new(token_repo.clone(), token_enc.clone()));
        let user_serv = Arc::new(UserService::new(user_repo.clone(), password_hasher.clone()));
        let authentication_serv = Arc::new(AuthenticationService::new(
            user_repo.clone(),
            password_hasher.clone(),
            token_serv.clone(),
        ));
        let authorization_serv = Arc::new(AuthorizationService::new(token_serv.clone()));

        Container {
            event_pub,

            role_repo,
            token_repo,
            user_repo,

            password_hasher,
            token_enc,

            token_serv,
            user_serv,
            authentication_serv,
            authorization_serv,
        }
    }

    pub async fn subscribe<ES>(&self, _event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber,
    {
        Ok(())
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn role_repo(&self) -> &dyn RoleRepository {
        self.role_repo.as_ref()
    }

    pub fn token_repo(&self) -> &dyn TokenRepository {
        self.token_repo.as_ref()
    }

    pub fn user_repo(&self) -> &dyn UserRepository {
        self.user_repo.as_ref()
    }

    // Services
    pub fn password_hasher(&self) -> &dyn PasswordHasher {
        self.password_hasher.as_ref()
    }

    pub fn token_enc(&self) -> &dyn TokenEncoder {
        self.token_enc.as_ref()
    }

    pub fn token_serv(&self) -> &TokenService {
        &self.token_serv
    }

    pub fn user_serv(&self) -> &UserService {
        &self.user_serv
    }

    pub fn authentication_serv(&self) -> &AuthenticationService {
        &self.authentication_serv
    }

    pub fn authorization_serv(&self) -> &AuthorizationService {
        &self.authorization_serv
    }
}
