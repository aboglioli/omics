use std::str::FromStr;

use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Status {
    #[serde(rename = "waiting-for-payment")]
    WaitingForPayment,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::WaitingForPayment => "waiting-for-payment".to_owned(),
            Status::Active => "active".to_owned(),
            Status::Inactive => "inactive".to_owned(),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "waiting-for-payment" => Ok(Status::WaitingForPayment),
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            _ => Err(Error::new("subscription_status", "invalid")),
        }
    }
}

impl Status {
    pub fn init() -> Self {
        Status::WaitingForPayment
    }

    pub fn wait_for_payment(&self) -> Result<Self> {
        match self {
            Status::Active => Ok(Status::WaitingForPayment),
            _ => Err(Error::new("subscription", "not_active")),
        }
    }

    pub fn pay(&self) -> Result<Self> {
        match self {
            Status::WaitingForPayment | Status::Active => Ok(Status::Active),
            _ => Err(Error::new("subscription", "not_waiting_payment")),
        }
    }

    pub fn close(&self) -> Result<Self> {
        match self {
            _ => Ok(Status::Inactive),
        }
    }
}
