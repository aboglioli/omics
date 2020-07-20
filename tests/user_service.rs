use std::rc::Rc;

use omics::{
    common::{error::Error, model::Entity},
    identity::{
        application::user::UserService,
        domain::{
            role::Role,
            token::{TokenService, TokenServiceImpl},
            user::{AuthService, AuthServiceImpl, User, UserRepository},
        },
        infrastructure::{mocks::*, persistence::inmem::*},
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
    user_serv: UserService<
        InMemUserRepository,
        InMemEventPublisher,
        AuthServiceImpl<
            InMemUserRepository,
            TokenServiceImpl<FakeTokenEncoder, InMemTokenRepository>,
            FakePasswordHasher,
        >,
        InMemRoleRepository,
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

        let user_serv = UserService::new(
            Rc::clone(&user_repo),
            Rc::clone(&event_pub),
            Rc::clone(&auth_serv),
            Rc::clone(&role_repo),
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
            user_serv,
        }
    }
}

#[test]
fn get_by_id() -> Result<(), Error> {
    let c = Container::new();

    let user_id = c.user_repo.next_id()?;
    let mut user = User::new(
        user_id.clone(),
        "user12",
        "user@email.com",
        &c.auth_serv.generate_password("user123")?,
        &Role::new("user".to_owned(), "User")?,
    )?;
    c.user_repo.save(&mut user)?;

    let found_user = c.user_serv.get_by_id(user_id)?;
    assert_eq!(found_user.id(), user.id());
    assert_eq!(found_user.username().value(), "user12");
    assert_eq!(found_user.email().value(), "user@email.com");

    Ok(())
}
