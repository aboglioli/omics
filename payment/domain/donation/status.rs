use std::str::FromStr;

use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Status {
    #[serde(rename = "waiting-for-payment")]
    WaitingForPayment,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "charged")]
    Charged,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::WaitingForPayment => "waiting-for-payment".to_owned(),
            Status::Paid => "paid".to_owned(),
            Status::Charged => "charged".to_owned(),
            Status::Cancelled => "cancelled".to_owned(),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "waiting-for-payment" => Ok(Status::WaitingForPayment),
            "paid" => Ok(Status::Paid),
            "charged" => Ok(Status::Charged),
            "cancelled" => Ok(Status::Cancelled),
            _ => Err(Error::new("donation_status", "invalid")),
        }
    }
}

impl Status {
    pub fn init() -> Self {
        Status::WaitingForPayment
    }

    pub fn pay(&self) -> Result<Self> {
        match self {
            Status::WaitingForPayment => Ok(Status::Paid),
            _ => Err(Error::new("donation_status", "not_waiting_payment")),
        }
    }

    pub fn charge(&self) -> Result<Self> {
        match self {
            Status::Paid => Ok(Status::Charged),
            _ => Err(Error::new("donation_status", "not_paid")),
        }
    }

    pub fn cancel(&self) -> Result<Self> {
        match self {
            Status::WaitingForPayment => Ok(Status::Cancelled),
            _ => Err(Error::new("donation_status", "not_waiting_payment")),
        }
    }
}
