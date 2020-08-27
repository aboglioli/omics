use common::error::Error;
use common::result::Result;

use crate::domain::content_manager::ContentManagerId;

#[derive(Debug, Clone)]
pub enum Status {
    Draft,
    WaitingApproval,
    Published { admin_id: ContentManagerId },
    Rejected { admin_id: ContentManagerId },
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

    pub fn approve(&self, content_manager_id: ContentManagerId) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Published {
                admin_id: content_manager_id,
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }

    pub fn reject(&self, content_manager_id: ContentManagerId) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Rejected {
                admin_id: content_manager_id,
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }
}
