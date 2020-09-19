use std::str::FromStr;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "other" => Ok(Gender::Other),
            _ => Err(Error::new("gender", "invalid")),
        }
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_owned(),
            Gender::Female => "female".to_owned(),
            Gender::Other => "other".to_owned(),
        }
    }
}
