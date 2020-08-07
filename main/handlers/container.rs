use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use common::event::InMemEventBus;
use identity::domain::token::TokenService;
use identity::domain::user::AuthService;
use identity::infrastructure::mocks::{FakePasswordHasher, FakeTokenEncoder};
use identity::infrastructure::persistence::inmem::{InMemTokenRepository, InMemUserRepository};

pub struct Container {
    event_bus: InMemEventBus,

    user_repo: InMemUserRepository,
    token_repo: InMemTokenRepository,

    password_hasher: FakePasswordHasher,
    token_enc: FakeTokenEncoder,
}

impl Container {
    pub fn new() -> Container {
        let event_bus = InMemEventBus::new();

        let user_repo = InMemUserRepository::new();
        let token_repo = InMemTokenRepository::new();

        let password_hasher = FakePasswordHasher::new();
        let token_enc = FakeTokenEncoder::new();

        Container {
            event_bus,

            user_repo,
            token_repo,

            password_hasher,
            token_enc,
        }
    }

    pub fn event_bus(&self) -> &InMemEventBus {
        &self.event_bus
    }

    pub fn user_repo(&self) -> &InMemUserRepository {
        &self.user_repo
    }

    pub fn token_repo(&self) -> &InMemTokenRepository {
        &self.token_repo
    }

    pub fn password_hasher(&self) -> &FakePasswordHasher {
        &self.password_hasher
    }

    pub fn token_enc(&self) -> &FakeTokenEncoder {
        &self.token_enc
    }

    pub fn auth_serv(
        &self,
    ) -> AuthService<
        '_,
        InMemUserRepository,
        FakePasswordHasher,
        InMemTokenRepository,
        FakeTokenEncoder,
    > {
        AuthService::new(self.user_repo(), self.token_serv(), self.password_hasher())
    }

    pub fn token_serv(&self) -> TokenService<'_, InMemTokenRepository, FakeTokenEncoder> {
        TokenService::new(self.token_repo(), self.token_enc())
    }
}

pub fn with_container(
    container: Arc<Container>,
) -> impl Filter<Extract = (Arc<Container>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&container))
}
