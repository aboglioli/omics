use common::error::Error;
use common::result::Result;

use crate::domain::user::UserId;

#[derive(Debug, Clone)]
pub enum Status {
    Draft,
    WaitingApproval,
    Published { admin_id: UserId },
    Rejected { admin_id: UserId },
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Draft => "draft".to_owned(),
            Status::WaitingApproval => "waiting-approval".to_owned(),
            Status::Published { .. } => "published".to_owned(),
            Status::Rejected { .. } => "rejected".to_owned(),
        }
    }
}

impl Status {
    pub fn draft(&self) -> Result<Self> {
        match self {
            Status::Draft => Err(Error::new("publication", "already_draft")),
            _ => Ok(Status::Draft),
        }
    }

    pub fn publish(&self) -> Result<Self> {
        match self {
            Status::Draft => Ok(Status::WaitingApproval),
            _ => Err(Error::new("publication", "not_a_draft")),
        }
    }

    pub fn approve(&self, user_id: UserId) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Published { admin_id: user_id }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }

    pub fn reject(&self, user_id: UserId) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Rejected { admin_id: user_id }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }
}
