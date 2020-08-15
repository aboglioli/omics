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
