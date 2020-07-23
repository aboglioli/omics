use common::error::Error;
use common::model::{Entity, ID};

pub type UserID = String;

pub struct User {
    id: ID<UserID>,
    name: String,
}

impl User {
    pub fn new(id: UserID, name: &str) -> Result<User, Error> {
        Ok(User {
            id: ID::new(id),
            name: name.to_owned(),
        })
    }
}

impl Entity<UserID> for User {
    fn id(&self) -> &ID<UserID> {
        &self.id
    }
}
