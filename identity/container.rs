use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::role::RoleRepository;
use crate::domain::token::{TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, UserRepository, UserService,
};

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    role_repo: Arc<dyn RoleRepository + Sync + Send>,
    token_repo: Arc<dyn TokenRepository + Sync + Send>,
    user_repo: Arc<dyn UserRepository + Sync + Send>,

    password_hasher: Arc<dyn PasswordHasher + Sync + Send>,
    token_enc: Arc<dyn TokenEncoder + Sync + Send>,

    token_serv: Arc<TokenService>,
    user_serv: Arc<UserService>,
    authentication_serv: Arc<AuthenticationService>,
    authorization_serv: Arc<AuthorizationService>,
}

impl<EPub: EventPublisher> Container<EPub> {
    pub fn new(
        event_pub: Arc<EPub>,

        role_repo: Arc<dyn RoleRepository + Sync + Send>,
        token_repo: Arc<dyn TokenRepository + Sync + Send>,
        user_repo: Arc<dyn UserRepository + Sync + Send>,

        password_hasher: Arc<dyn PasswordHasher + Sync + Send>,
        token_enc: Arc<dyn TokenEncoder + Sync + Send>,
    ) -> Self {
        let token_serv = Arc::new(TokenService::new(
            Arc::clone(&token_repo),
            Arc::clone(&token_enc),
        ));
        let user_serv = Arc::new(UserService::new(
            Arc::clone(&user_repo),
            Arc::clone(&password_hasher),
        ));
        let authentication_serv = Arc::new(AuthenticationService::new(
            Arc::clone(&user_repo),
            Arc::clone(&password_hasher),
            Arc::clone(&token_serv),
        ));
        let authorization_serv = Arc::new(AuthorizationService::new(
            Arc::clone(&user_repo),
            Arc::clone(&token_serv),
        ));

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
