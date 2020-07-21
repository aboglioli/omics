use std::rc::Rc;

use common::{error::Error, model::Entity};
use identity::{
    application::user::UserService,
    domain::{token::*, user::*},
    infrastructure::{
        mocks::{self, *},
        persistence::inmem::*,
    },
};

struct Container {
    user_repo: Rc<InMemUserRepository>,
    event_pub: Rc<InMemEventPublisher>,
    password_hasher: Rc<FakePasswordHasher>,
    token_enc: Rc<FakeTokenEncoder>,
    token_repo: Rc<InMemTokenRepository>,
    token_serv: Rc<TokenServiceImpl<FakeTokenEncoder, InMemTokenRepository>>,
    auth_serv: Rc<
        AuthServiceImpl<
            InMemUserRepository,
            TokenServiceImpl<FakeTokenEncoder, InMemTokenRepository>,
            FakePasswordHasher,
        >,
    >,
    role_repo: Rc<InMemRoleRepository>,
    validation_repo: Rc<InMemValidationRepository>,
    user_serv: UserService<
        InMemUserRepository,
        InMemEventPublisher,
        AuthServiceImpl<
            InMemUserRepository,
            TokenServiceImpl<FakeTokenEncoder, InMemTokenRepository>,
            FakePasswordHasher,
        >,
        InMemRoleRepository,
        InMemValidationRepository,
    >,
}

impl Container {
    fn new() -> Container {
        let user_repo = Rc::new(InMemUserRepository::new());
        let event_pub = Rc::new(InMemEventPublisher::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_enc = Rc::new(FakeTokenEncoder::new());
        let token_repo = Rc::new(InMemTokenRepository::new());
        let token_serv = Rc::new(TokenServiceImpl::new(
            Rc::clone(&token_enc),
            Rc::clone(&token_repo),
        ));
        let auth_serv = Rc::new(AuthServiceImpl::new(
            Rc::clone(&user_repo),
            Rc::clone(&token_serv),
            Rc::clone(&password_hasher),
        ));
        let role_repo = Rc::new(InMemRoleRepository::new());
        let validation_repo = Rc::new(InMemValidationRepository::new());

        let user_serv = UserService::new(
            Rc::clone(&user_repo),
            Rc::clone(&event_pub),
            Rc::clone(&auth_serv),
            Rc::clone(&role_repo),
            Rc::clone(&validation_repo),
        );

        Container {
            user_repo,
            event_pub,
            password_hasher,
            token_enc,
            token_repo,
            token_serv,
            auth_serv,
            role_repo,
            validation_repo,
            user_serv,
        }
    }
}

#[test]
fn get_by_id() -> Result<(), Error> {
    let c = Container::new();

    let mut user = mocks::user1()?;
    c.user_repo.save(&mut user)?;

    let found_user = c.user_serv.get_by_id(&user.id().value())?;
    assert_eq!(found_user.id(), user.id());
    assert_eq!(found_user.identity().username().value(), "username");
    assert_eq!(found_user.identity().email().value(), "username@email.com");

    Ok(())
}
