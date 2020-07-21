use common::error::Error;

use crate::domain::user::Fullname;

#[derive(Debug, Clone)]
pub struct Person {
    fullname: Fullname,
}

impl Person {
    pub fn new(fullname: Fullname) -> Result<Person, Error> {
        Ok(Person { fullname })
    }

    pub fn fullname(&self) -> &Fullname {
        &self.fullname
    }
}
