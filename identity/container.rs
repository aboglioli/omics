use std::sync::Arc;

use async_trait::async_trait;

use common::container::Container;
use common::event::EventPublisher;

use crate::domain::role::{PermissionRepository, RoleRepository};
use crate::domain::token::{TokenEncoder, TokenRepository, TokenService};
use crate::domain::user::{
    AuthenticationService, AuthorizationService, PasswordHasher, UserRepository, UserService,
};

pub struct IdentityContainer<EPub> {
    event_pub: Arc<EPub>,

    permission_repo: Arc<dyn PermissionRepository>,
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

impl<EPub> IdentityContainer<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,

        permission_repo: Arc<dyn PermissionRepository>,
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

        IdentityContainer {
            event_pub,

            permission_repo,
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

    pub fn permission_repo(&self) -> &dyn PermissionRepository {
        self.permission_repo.as_ref()
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

#[async_trait]
impl<EPub> Container for IdentityContainer<EPub> where EPub: Sync + Send {}
