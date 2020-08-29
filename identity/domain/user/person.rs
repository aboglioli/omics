use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::user::{Fullname, Gender};

#[derive(Debug, Clone)]
pub struct Person {
    fullname: Fullname,
    birthdate: Option<DateTime<Utc>>,
    gender: Option<Gender>,
}

impl Person {
    pub fn new(
        fullname: Fullname,
        birthdate: Option<DateTime<Utc>>,
        gender: Option<Gender>,
    ) -> Result<Self> {
        Ok(Person {
            fullname,
            birthdate,
            gender,
        })
    }

    pub fn fullname(&self) -> &Fullname {
        &self.fullname
    }

    pub fn birthdate(&self) -> Option<&DateTime<Utc>> {
        self.birthdate.as_ref()
    }

    pub fn gender(&self) -> Option<&Gender> {
        self.gender.as_ref()
    }
}
