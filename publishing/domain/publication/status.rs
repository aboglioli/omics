pub enum PublicationStatus {
    Draft,
    WaitingApproval,
    Published,
    Rejected,
}

impl ToString for PublicationStatus {
    fn to_string(&self) -> String {
        match self {
            PublicationStatus::Draft => "draft".to_owned(),
            PublicationStatus::WaitingApproval => "waiting-approval".to_owned(),
            PublicationStatus::Published => "published".to_owned(),
            PublicationStatus::Rejected => "rejected".to_owned(),
        }
    }
}
