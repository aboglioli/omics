use crate::domain::content_manager::ContentManager;

#[derive(Debug, Clone)]
pub enum Status {
    Draft,
    WaitingApproval,
    Published { admin: ContentManager },
    Rejected { admin: ContentManager },
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
