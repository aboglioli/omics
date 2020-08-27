use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub enum Status {
    WaitingPayment,
    Paid,
    Rejected,
    Cancelled,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::WaitingPayment => "waiting-payment".to_owned(),
            Status::Paid => "paid".to_owned(),
            Status::Rejected => "rejected".to_owned(),
            Status::Cancelled => "cancelled".to_owned(),
        }
    }
}

impl Status {
    pub fn pay(&self) -> Result<Self> {
        match self {
            Status::WaitingPayment => Ok(Status::Paid),
            _ => Err(Error::new("payment", "not_waiting_payment")),
        }
    }

    pub fn reject(&self) -> Result<Self> {
        match self {
            Status::WaitingPayment => Ok(Status::Rejected),
            _ => Err(Error::new("payment", "not_waiting_payment")),
        }
    }

    pub fn cancel(&self) -> Result<Self> {
        match self {
            Status::WaitingPayment => Ok(Status::Cancelled),
            _ => Err(Error::new("payment", "not_waiting_payment")),
        }
    }
}
