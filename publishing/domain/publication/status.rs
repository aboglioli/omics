use std::str::FromStr;

use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::interaction::Comment;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "waiting-approval")]
    WaitingApproval,
    #[serde(rename = "published")]
    Published {
        admin_id: Option<UserId>,
        comment: Option<Comment>,
    },
    #[serde(rename = "rejected")]
    Rejected {
        admin_id: Option<UserId>,
        comment: Option<Comment>,
    },
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

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "draft" => Ok(Status::Draft),
            "waiting-approval" => Ok(Status::WaitingApproval),
            "published" => Ok(Status::Published {
                admin_id: None,
                comment: None,
            }),
            "rejected" => Ok(Status::Rejected {
                admin_id: None,
                comment: None,
            }),
            _ => Err(Error::new("publication_status", "invalid")),
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
                admin_id: Some(user_id),
                comment: Some(comment),
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }

    pub fn reject(&self, user_id: UserId, comment: Comment) -> Result<Self> {
        match self {
            Status::WaitingApproval => Ok(Status::Rejected {
                admin_id: Some(user_id),
                comment: Some(comment),
            }),
            _ => Err(Error::new("publication", "not_waiting_approval")),
        }
    }
}
