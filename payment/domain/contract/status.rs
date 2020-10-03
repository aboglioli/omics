use common::error::Error;
use common::result::Result;
use identity::domain::user::UserId;

#[derive(Debug, Clone)]
pub enum Status {
    Requested,
    Approved { admin_id: UserId },
    Rejected { admin_id: UserId },
    Cancelled,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Requested => "requested".to_owned(),
            Status::Approved { .. } => "approved".to_owned(),
            Status::Rejected { .. } => "rejected".to_owned(),
            Status::Cancelled => "cancelled".to_owned(),
        }
    }
}

impl Status {
    pub fn init() -> Self {
        Status::Requested
    }

    pub fn approve(&self, user_id: UserId) -> Result<Self> {
        match self {
            Status::Requested => Ok(Status::Approved { admin_id: user_id }),
            _ => Err(Error::new("contract", "not_requested")),
        }
    }

    pub fn reject(&self, user_id: UserId) -> Result<Self> {
        match self {
            Status::Requested => Ok(Status::Rejected { admin_id: user_id }),
            _ => Err(Error::new("contract", "not_requested")),
        }
    }

    pub fn cancel(&self) -> Result<Self> {
        Ok(Status::Cancelled)
    }
}
