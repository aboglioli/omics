use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn from(date_str: &str) -> Result<Gender> {
        match date_str {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err(Error::new("gender", "invalid")),
        }
    }
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => "male".to_owned(),
            Gender::Female => "female".to_owned(),
        }
    }
}