use common::error::Error;
use common::result::Result;
use shared::domain::user::UserId;

use crate::domain::interaction::Comment;

#[derive(Debug, Clone)]
pub enum Status {
    Draft,
    WaitingApproval,
    Published { admin_id: UserId, comment: Comment },
    Rejected { admin_id: UserId, comment: Comment },
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

    pub fn approve(&self, user_id: UserId, comment: Comment) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Published {
                admin_id: user_id,
                comment,
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }

    pub fn reject(&self, user_id: UserId, comment: Comment) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Rejected {
                admin_id: user_id,
                comment,
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }
}
