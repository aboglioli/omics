use crate::common::event::Event;
use crate::identity::domain::user::{Person, UserID};

pub struct UserUpdated {
    pub id: UserID,
    pub person: Person,
}

impl UserUpdated {
    pub fn new(id: UserID, person: Person) -> UserUpdated {
        UserUpdated { id, person }
    }
}

impl Event for UserUpdated {
    fn code(&self) -> &str {
        "user-created"
    }

    fn payload(&self) -> Vec<u8> {
        let res = self.id.clone() + self.person.name();
        res.as_bytes().to_vec()
    }
}
