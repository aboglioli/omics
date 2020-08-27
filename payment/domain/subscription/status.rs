use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Status {
    Active,
    Inactive,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Active => "active".to_owned(),
            Status::Inactive => "inactive".to_owned(),
        }
    }
}

impl Status {
    pub fn close(&self) -> Result<Self> {
        match self {
            Status::Active => Ok(Status::Inactive),
            _ => Err(Error::new("subscription", "not_active")),
        }
    }
}
