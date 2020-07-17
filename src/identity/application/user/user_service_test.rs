use super::*;

use crate::common::error::Error;
use crate::common::model::Entity;
use crate::identity::domain::{
    role::Role,
    token::TokenService,
    user::{AuthenticationService, AuthorizationService, User, UserRepository},
};
use crate::identity::infrastructure::{mocks::*, persistence::inmem::*};

// impl TestBed<'_> {
//     fn new() -> TestBed<'static> {
//         let user_repo = InMemUserRepository::new();
//         let event_pub = InMemEventPublisher::new();
//         let password_hasher = FakePasswordHasher::new();
//         let token_encoder = FakeTokenEncoder::new();
//         let token_repository = InMemTokenRepository::new();
//         let token_serv = TokenService::new(token_encoder, token_repository);
//         let authentication_serv =
//             AuthenticationService::new(&user_repo, &password_hasher, &token_serv);
//         let authorization_serv = AuthorizationService::new(&user_repo, &password_hasher);
//         let role_repo = InMemRoleRepository::new();
//
//         TestBed {
//             user_repo,
//             event_pub,
//             password_hasher,
//             token_encoder,
//             token_repository,
//             token_serv,
//             authentication_serv,
//             authorization_serv,
//             role_repo,
//         }
//     }
// }

#[test]
fn get_by_id() -> Result<(), Error> {
    let user_repo = InMemUserRepository::new();
    let event_pub = InMemEventPublisher::new();
    let password_hasher = FakePasswordHasher::new();
    let token_encoder = FakeTokenEncoder::new();
    let token_repository = InMemTokenRepository::new();
    let token_serv = TokenService::new(token_encoder, token_repository);
    let authentication_serv = AuthenticationService::new(&user_repo, &password_hasher, &token_serv);
    let authorization_serv = AuthorizationService::new(&user_repo, &password_hasher);
    let role_repo = InMemRoleRepository::new();

    let user_serv = UserService::new(
        &user_repo,
        &event_pub,
        &authentication_serv,
        &authorization_serv,
        &role_repo,
    );

    let user_id = user_repo.next_id()?;
    let mut user = User::new(
        user_id.clone(),
        "user12",
        "user@email.com",
        &authorization_serv.generate_password("user123")?,
        &Role::new("user".to_owned(), "User")?,
    )?;
    user_repo.save(&mut user)?;

    let found_user = user_serv.get_by_id(user_id)?;
    assert_eq!(found_user.id(), user.id());
    assert_eq!(found_user.username().value(), "user12");
    assert_eq!(found_user.email().value(), "user@email.com");

    Ok(())
}
