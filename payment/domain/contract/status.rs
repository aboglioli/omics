use crate::domain::admin::AdminId;

#[derive(Debug, Clone)]
pub enum Status {
    Requested,
    Approved { admin_id: AdminId },
    Rejected { admin_id: AdminId },
    Cancelled,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Requested => "requested".to_owned(),
            Status::Approved { .. } => "approved".to_owned(),
            Status::Rejected { .. } => "rejected".to_owned(),
            Status::Cancelled => "cancelled".to_owned(),
        }
    }
}
