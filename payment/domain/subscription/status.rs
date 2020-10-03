use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Status {
    WaitingForPayment,
    Active,
    Inactive,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::WaitingForPayment => "waiting-payment".to_owned(),
            Status::Active => "active".to_owned(),
            Status::Inactive => "inactive".to_owned(),
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
