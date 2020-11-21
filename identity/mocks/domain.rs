use crate::domain::role::{Name, Permission, Role, RoleId};
use crate::domain::user::{
    Email, Fullname, Identity, Password, PasswordHasher, Person, Provider, User, UserId, Username,
};
use crate::mocks::FakePasswordHasher;

pub fn user(
    user_id: &str,
    username: &str,
    email: &str,
    password: &str,
    validated: bool,
    name: Option<&str>,
    lastname: Option<&str>,
    role_id: &str,
) -> User {
    let ph = FakePasswordHasher::new();
    let mut user = User::new(
        UserId::new(user_id).unwrap(),
        Identity::new(
            Provider::Local,
            Username::new(username).unwrap(),
            Email::new(email).unwrap(),
            Some(Password::new(&ph.hash(password).unwrap()).unwrap()),
        )
        .unwrap(),
        RoleId::new(role_id).unwrap(),
    )
    .unwrap();

    if validated {
        let validation = user.validation().cloned().unwrap();
        user.validate(&validation).unwrap();
    }

    if name.is_some() && lastname.is_some() {
        user.set_person(
            Person::new(
                Fullname::new(name.unwrap(), lastname.unwrap()).unwrap(),
                None,
                None,
                None,
                None,
            )
            .unwrap(),
        )
        .unwrap();
    }

    user
}

pub fn role(name: &str) -> Role {
    let mut role = Role::new(Name::new(name).unwrap()).unwrap();
    role.set_permissions(vec![Permission::new("*", "Superpowers").unwrap()])
        .unwrap();
    role
}
