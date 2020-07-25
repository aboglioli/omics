use common::error::Error;

use crate::domain::role::{Role, RoleId};
use crate::domain::user::{
    Email, Identity, Password, PasswordHasher, Provider, User, UserId, Username,
};
use crate::infrastructure::mocks::FakePasswordHasher;

pub fn user1() -> Result<User, Error> {
    let ph = FakePasswordHasher::new();
    User::new(
        UserId::from("user123"),
        Identity::new(
            Provider::Local,
            Username::new("username")?,
            Email::new("username@email.com")?,
            Some(Password::new(&ph.hash("P@asswd!")?)?),
        )?,
        RoleId::from("user"),
    )
}
