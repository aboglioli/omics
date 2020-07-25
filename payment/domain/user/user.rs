use common::error::Error;
use common::model::AggregateRoot;

pub type UserID = String;

pub struct User {
    base: AggregateRoot<UserID>,
    name: String,
}

impl User {
    pub fn new(id: UserID, name: &str) -> Result<User, Error> {
        Ok(User {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
        })
    }
}
