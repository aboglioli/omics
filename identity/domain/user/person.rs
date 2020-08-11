use common::result::Result;

use crate::domain::user::Fullname;

#[derive(Debug, Clone)]
pub struct Person {
    fullname: Fullname,
}

impl Person {
    pub fn new(fullname: Fullname) -> Result<Self> {
        Ok(Person { fullname })
    }

    pub fn fullname(&self) -> &Fullname {
        &self.fullname
    }
}
