use std::rc::Rc;

use common::{error::Error, event::EventPublisher};
use identity::{
    application::user::UserService,
    domain::{role::*, token::*, user::*, validation::*},
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
    token_serv: Rc<TokenService>,
    auth_serv: Rc<AuthService>,
    role_repo: Rc<InMemRoleRepository>,
    validation_repo: Rc<InMemValidationRepository>,
    user_serv: UserService,
}

impl Container {
    fn new() -> Container {
        let user_repo = Rc::new(InMemUserRepository::new());
        let event_pub = Rc::new(InMemEventPublisher::new());
        let password_hasher = Rc::new(FakePasswordHasher::new());
        let token_enc = Rc::new(FakeTokenEncoder::new());
        let token_repo = Rc::new(InMemTokenRepository::new());
        let token_serv = Rc::new(TokenService::new(
            Rc::clone(&token_enc) as Rc<dyn TokenEncoder>,
            Rc::clone(&token_repo) as Rc<dyn TokenRepository>,
        ));
        let auth_serv = Rc::new(AuthService::new(
            Rc::clone(&user_repo) as Rc<dyn UserRepository>,
            Rc::clone(&token_serv),
            Rc::clone(&password_hasher) as Rc<dyn PasswordHasher>,
        ));
        let role_repo = Rc::new(InMemRoleRepository::new());
        let validation_repo = Rc::new(InMemValidationRepository::new());

        let user_serv = UserService::new(
            Rc::clone(&user_repo) as Rc<dyn UserRepository>,
            Rc::clone(&event_pub) as Rc<dyn EventPublisher>,
            Rc::clone(&auth_serv),
            Rc::clone(&role_repo) as Rc<dyn RoleRepository>,
            Rc::clone(&validation_repo) as Rc<dyn ValidationRepository>,
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

    let found_user = c.user_serv.get_by_id(&user.base().id())?;
    assert_eq!(found_user.base().id(), user.base().id());
    assert_eq!(found_user.identity().username().value(), "username");
    assert_eq!(found_user.identity().email().value(), "username@email.com");

    Ok(())
}
