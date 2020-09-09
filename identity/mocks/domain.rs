use crate::domain::role::{Role, RoleId};
use crate::domain::user::{
    Email, Fullname, Identity, Password, PasswordHasher, Person, Provider, User, UserId, Username,
};
use crate::mocks::FakePasswordHasher;

pub fn user1() -> User {
    let ph = FakePasswordHasher::new();
    User::new(
        UserId::new("#user1").unwrap(),
        Identity::new(
            Provider::Local,
            Username::new("user-one").unwrap(),
            Email::new("user@one.com").unwrap(),
            Some(Password::new(&ph.hash("P@asswd!").unwrap()).unwrap()),
        )
        .unwrap(),
        user_role().base().id().clone(),
    )
    .unwrap()
}

pub fn user2() -> User {
    let ph = FakePasswordHasher::new();
    User::new(
        UserId::new("#user2").unwrap(),
        Identity::new(
            Provider::Local,
            Username::new("user-two").unwrap(),
            Email::new("user@two.com").unwrap(),
            Some(Password::new(&ph.hash("P@asswd!").unwrap()).unwrap()),
        )
        .unwrap(),
        user_role().base().id().clone(),
    )
    .unwrap()
}

pub fn validated_user1() -> User {
    let mut user = user1();

    let validation = user.validation().cloned().unwrap();
    user.validate(&validation).unwrap();

    user
}

pub fn validated_user2() -> User {
    let mut user = user2();

    let validation = user.validation().cloned().unwrap();
    user.validate(&validation).unwrap();

    user
}

pub fn admin1() -> User {
    let ph = FakePasswordHasher::new();
    User::new(
        UserId::new("#admin1").unwrap(),
        Identity::new(
            Provider::Local,
            Username::new("admin-1").unwrap(),
            Email::new("admin.1@system.com").unwrap(),
            Some(Password::new(&ph.hash("P@asswd!").unwrap()).unwrap()),
        )
        .unwrap(),
        admin_role().base().id().clone(),
    )
    .unwrap()
}

pub fn user_role() -> Role {
    Role::new(RoleId::new("user").unwrap(), "User").unwrap()
}

pub fn admin_role() -> Role {
    Role::new(RoleId::new("admin").unwrap(), "Administrator").unwrap()
}

pub fn person1() -> Person {
    Person::new(
        Fullname::new("User", "One").unwrap(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
}
