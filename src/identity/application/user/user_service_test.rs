use std::rc::Rc;

use super::*;
use crate::common::error::Error;
use crate::common::model::Entity;
use crate::identity::domain::{
    role::Role,
    token::TokenService,
    user::{AuthenticationService, AuthorizationService, User, UserRepository},
};
use crate::identity::infrastructure::{mocks::*, persistence::inmem::*};

struct Container {
    user_repo: Rc<InMemUserRepository>,
    event_pub: Rc<InMemEventPublisher>,
    password_hasher: Rc<FakePasswordHasher>,
    token_serv: Rc<TokenService<FakeTokenEncoder, InMemTokenRepository>>,
    authentication_serv: Rc<
        AuthenticationService<
            InMemUserRepository,
            FakePasswordHasher,
            FakeTokenEncoder,
            InMemTokenRepository,
        >,
    >,
    authorization_serv: Rc<AuthorizationService<InMemUserRepository, FakePasswordHasher>>,
    role_repo: Rc<InMemRoleRepository>,
    user_serv: UserService<
        InMemUserRepository,
        InMemEventPublisher,
        FakePasswordHasher,
        FakeTokenEncoder,
        InMemTokenRepository,
        InMemRoleRepository,
    >,
}

impl Container {
    fn new() -> Container {
        let user_repo = Rc::new(InMemUserRepository::new());
        let event_pub = Rc::new(InMemEventPublisher::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_serv = Rc::new(TokenService::new(
            FakeTokenEncoder::new(),
            InMemTokenRepository::new(),
        ));
        let authentication_serv = Rc::new(AuthenticationService::new(
            Rc::clone(&user_repo),
            Rc::clone(&password_hasher),
            Rc::clone(&token_serv),
        ));
        let authorization_serv = Rc::new(AuthorizationService::new(
            Rc::clone(&user_repo),
            Rc::clone(&password_hasher),
        ));
        let role_repo = Rc::new(InMemRoleRepository::new());

        let user_serv = UserService::new(
            Rc::clone(&user_repo),
            Rc::clone(&event_pub),
            Rc::clone(&authentication_serv),
            Rc::clone(&authorization_serv),
            Rc::clone(&role_repo),
        );

        Container {
            user_repo,
            event_pub,
            password_hasher,
            token_serv,
            authentication_serv,
            authorization_serv,
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
        &c.authorization_serv.generate_password("user123")?,
        &Role::new("user".to_owned(), "User")?,
    )?;
    c.user_repo.save(&mut user)?;

    let found_user = c.user_serv.get_by_id(user_id)?;
    assert_eq!(found_user.id(), user.id());
    assert_eq!(found_user.username().value(), "user12");
    assert_eq!(found_user.email().value(), "user@email.com");

    Ok(())
}
