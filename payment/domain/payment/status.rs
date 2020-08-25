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
