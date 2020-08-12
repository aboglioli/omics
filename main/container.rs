use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use common::event::inmem::InMemEventBus;
use identity::domain::token::TokenService;
use identity::domain::user::{AuthenticationService, AuthorizationService, UserService};
use identity::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};
use identity::mocks::{FakePasswordHasher, FakeTokenEncoder};

pub struct Container {
    event_bus: InMemEventBus,

    #[allow(dead_code)]
    role_repo: InMemRoleRepository,
    token_repo: InMemTokenRepository,
    user_repo: InMemUserRepository,

    password_hasher: FakePasswordHasher,
    token_enc: FakeTokenEncoder,
}

impl Container {
    pub fn new() -> Container {
        let event_bus = InMemEventBus::new();

        let role_repo = InMemRoleRepository::new();
        let token_repo = InMemTokenRepository::new();
        let user_repo = InMemUserRepository::new();

        let password_hasher = FakePasswordHasher::new();
        let token_enc = FakeTokenEncoder::new();

        Container {
            event_bus,

            role_repo,
            token_repo,
            user_repo,

            password_hasher,
            token_enc,
        }
    }

    // Events
    pub fn event_bus(&self) -> &InMemEventBus {
        &self.event_bus
    }

    // Repositories
    #[allow(dead_code)]
    pub fn role_repo(&self) -> &InMemRoleRepository {
        &self.role_repo
    }

    pub fn token_repo(&self) -> &InMemTokenRepository {
        &self.token_repo
    }

    pub fn user_repo(&self) -> &InMemUserRepository {
        &self.user_repo
    }

    // Tools
    pub fn password_hasher(&self) -> &FakePasswordHasher {
        &self.password_hasher
    }

    pub fn token_enc(&self) -> &FakeTokenEncoder {
        &self.token_enc
    }

    // Services
    pub fn token_serv(&self) -> TokenService<'_, InMemTokenRepository, FakeTokenEncoder> {
        TokenService::new(self.token_repo(), self.token_enc())
    }

    pub fn authentication_serv(
        &self,
    ) -> AuthenticationService<
        '_,
        InMemUserRepository,
        FakePasswordHasher,
        InMemTokenRepository,
        FakeTokenEncoder,
    > {
        AuthenticationService::new(self.user_repo(), self.password_hasher(), self.token_serv())
    }

    pub fn authorization_serv(
        &self,
    ) -> AuthorizationService<'_, InMemUserRepository, InMemTokenRepository, FakeTokenEncoder> {
        AuthorizationService::new(self.user_repo(), self.token_serv())
    }

    pub fn user_serv(&self) -> UserService<'_, InMemUserRepository, FakePasswordHasher> {
        UserService::new(self.user_repo(), self.password_hasher())
    }
}

pub fn with_container(
    container: Arc<Container>,
) -> impl Filter<Extract = (Arc<Container>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&container))
}
