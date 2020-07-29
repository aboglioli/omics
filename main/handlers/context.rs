use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;

use common::event::InMemEventBus;
use identity::domain::token::TokenService;
use identity::domain::user::AuthService;
use identity::infrastructure::mocks::{FakePasswordHasher, FakeTokenEncoder};
use identity::infrastructure::persistence::inmem::{InMemTokenRepository, InMemUserRepository};

pub struct Context {
    event_bus: Arc<InMemEventBus>,

    user_repo: Arc<InMemUserRepository>,
    token_repo: Arc<InMemTokenRepository>,

    password_hasher: Arc<FakePasswordHasher>,
    token_enc: Arc<FakeTokenEncoder>,

    token_serv: Arc<TokenService<InMemTokenRepository, FakeTokenEncoder>>,
    auth_serv: Arc<
        AuthService<
            InMemUserRepository,
            FakePasswordHasher,
            InMemTokenRepository,
            FakeTokenEncoder,
        >,
    >,
}

impl Context {
    pub fn new() -> Context {
        let event_bus = Arc::new(InMemEventBus::new());

        let user_repo = Arc::new(InMemUserRepository::new());
        let token_repo = Arc::new(InMemTokenRepository::new());

        let password_hasher = Arc::new(FakePasswordHasher::new());
        let token_enc = Arc::new(FakeTokenEncoder::new());

        let token_serv = Arc::new(TokenService::new(token_repo.clone(), token_enc.clone()));
        let auth_serv = Arc::new(AuthService::new(
            user_repo.clone(),
            token_serv.clone(),
            password_hasher.clone(),
        ));

        Context {
            event_bus,

            user_repo,
            token_repo,

            password_hasher,
            token_enc,

            token_serv,
            auth_serv,
        }
    }

    pub fn event_bus(&self) -> Arc<InMemEventBus> {
        self.event_bus.clone()
    }

    pub fn auth_serv(
        &self,
    ) -> Arc<
        AuthService<
            InMemUserRepository,
            FakePasswordHasher,
            InMemTokenRepository,
            FakeTokenEncoder,
        >,
    > {
        self.auth_serv.clone()
    }

    pub fn user_repo(&self) -> Arc<InMemUserRepository> {
        self.user_repo.clone()
    }
}

pub fn with_context(
    ctx: Arc<Context>,
) -> impl Filter<Extract = (Arc<Context>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&ctx))
}
