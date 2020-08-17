use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::role::RoleRepository;
use crate::domain::token::{TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, UserRepository, UserService,
};

pub struct Container<EPub, RRepo, TRepo, URepo, PHasher, TEnc> {
    event_pub: Arc<EPub>,

    role_repo: Arc<RRepo>,
    token_repo: Arc<TRepo>,
    user_repo: Arc<URepo>,

    password_hasher: Arc<PHasher>,
    token_enc: Arc<TEnc>,

    token_serv: Arc<TokenService<TRepo, TEnc>>,
    user_serv: Arc<UserService<URepo, PHasher>>,
    authentication_serv: Arc<AuthenticationService<URepo, PHasher, TRepo, TEnc>>,
    authorization_serv: Arc<AuthorizationService<URepo, TRepo, TEnc>>,
}

impl<EPub, RRepo, TRepo, URepo, PHasher, TEnc> Container<EPub, RRepo, TRepo, URepo, PHasher, TEnc>
where
    EPub: EventPublisher,
    RRepo: RoleRepository,
    TRepo: TokenRepository,
    URepo: UserRepository,
    PHasher: PasswordHasher,
    TEnc: TokenEncoder,
{
    pub fn new(
        event_pub: Arc<EPub>,
        role_repo: Arc<RRepo>,
        token_repo: Arc<TRepo>,
        user_repo: Arc<URepo>,
        password_hasher: Arc<PHasher>,
        token_enc: Arc<TEnc>,
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

    pub fn role_repo(&self) -> &RRepo {
        &self.role_repo
    }

    pub fn token_repo(&self) -> &TRepo {
        &self.token_repo
    }

    pub fn user_repo(&self) -> &URepo {
        &self.user_repo
    }

    // Services
    pub fn password_hasher(&self) -> &PHasher {
        &self.password_hasher
    }

    pub fn token_enc(&self) -> &TEnc {
        &self.token_enc
    }

    pub fn token_serv(&self) -> &TokenService<TRepo, TEnc> {
        &self.token_serv
    }

    pub fn user_serv(&self) -> &UserService<URepo, PHasher> {
        &self.user_serv
    }

    pub fn authentication_serv(&self) -> &AuthenticationService<URepo, PHasher, TRepo, TEnc> {
        &self.authentication_serv
    }

    pub fn authorization_serv(&self) -> &AuthorizationService<URepo, TRepo, TEnc> {
        &self.authorization_serv
    }
}
