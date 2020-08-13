use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::role::RoleRepository;
use crate::domain::token::{TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, UserRepository, UserService,
};

pub struct Container<EPub, RRepo, TRepo, URepo, PHasher, TEnc> {
    event_pub: Arc<EPub>,

    role_repo: RRepo,
    token_repo: TRepo,
    user_repo: URepo,

    password_hasher: PHasher,
    token_enc: TEnc,
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
        role_repo: RRepo,
        token_repo: TRepo,
        user_repo: URepo,
        password_hasher: PHasher,
        token_enc: TEnc,
    ) -> Self {
        Container {
            event_pub,

            role_repo,
            token_repo,
            user_repo,

            password_hasher,
            token_enc,
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

    pub fn token_serv(&self) -> TokenService<'_, TRepo, TEnc> {
        TokenService::new(self.token_repo(), self.token_enc())
    }

    pub fn user_serv(&self) -> UserService<'_, URepo, PHasher> {
        UserService::new(self.user_repo(), self.password_hasher())
    }

    pub fn authentication_serv(&self) -> AuthenticationService<'_, URepo, PHasher, TRepo, TEnc> {
        AuthenticationService::new(self.user_repo(), self.password_hasher(), self.token_serv())
    }

    pub fn authorization_serv(&self) -> AuthorizationService<'_, URepo, TRepo, TEnc> {
        AuthorizationService::new(self.user_repo(), self.token_serv())
    }
}
