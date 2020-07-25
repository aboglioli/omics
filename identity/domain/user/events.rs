use common::event::Event;

use crate::domain::user::UserID;

#[derive(Debug)]
pub struct UserUpdated {
    pub id: UserID,
    pub name: String,
    pub lastname: String,
}

impl UserUpdated {
    pub fn new(id: UserID, name: &str, lastname: &str) -> UserUpdated {
        UserUpdated {
            id,
            name: name.to_owned(),
            lastname: lastname.to_owned(),
        }
    }
}

impl Event for UserUpdated {
    fn code(&self) -> &str {
        "user-created"
    }

    fn payload(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct UserRegistered {
    pub id: UserID,
    pub username: String,
    pub email: String,
}

impl UserRegistered {
    pub fn new(id: UserID, username: &str, email: &str) -> UserRegistered {
        UserRegistered {
            id,
            username: username.to_owned(),
            email: email.to_owned(),
        }
    }
}

impl Event for UserRegistered {
    fn code(&self) -> &str {
        "user-registered"
    }

    fn payload(&self) -> Vec<u8> {
        Vec::new()
    }
}
