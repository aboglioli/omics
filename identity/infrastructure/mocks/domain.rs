use crate::domain::role::{Role, RoleId};
use crate::domain::user::{
    Email, Identity, Password, PasswordHasher, Provider, User, UserId, Username,
};
use crate::infrastructure::mocks::FakePasswordHasher;

pub fn user1() -> User {
    let ph = FakePasswordHasher::new();
    User::new(
        UserId::new("user123").unwrap(),
        Identity::new(
            Provider::Local,
            Username::new("username").unwrap(),
            Email::new("username@email.com").unwrap(),
            Some(Password::new(&ph.hash("P@asswd!").unwrap()).unwrap()),
        )
        .unwrap(),
        Role::new(RoleId::new("user").unwrap(), "User").unwrap(),
    )
    .unwrap()
}
