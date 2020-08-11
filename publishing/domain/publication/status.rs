pub enum Status {
    Draft,
    WaitingApproval,
    Published,
    Rejected,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Draft => "draft".to_owned(),
            Status::WaitingApproval => "waiting-approval".to_owned(),
            Status::Published => "published".to_owned(),
            Status::Rejected => "rejected".to_owned(),
        }
    }
}
