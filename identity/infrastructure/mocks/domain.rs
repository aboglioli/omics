use common::error::Error;

use crate::domain::role::{Role, RoleID};
use crate::domain::user::{
    Email, Identity, Password, PasswordHasher, Provider, User, UserID, Username,
};
use crate::infrastructure::mocks::FakePasswordHasher;

pub fn user1() -> Result<User, Error> {
    let ph = FakePasswordHasher::new();
    User::new(
        UserID::from("user123"),
        Identity::new(
            Provider::new("local")?,
            Username::new("username")?,
            Email::new("username@email.com")?,
            Some(Password::new(&ph.hash("P@asswd!")?)?),
        )?,
        &Role::new(RoleID::from("user"), "User")?,
    )
}
