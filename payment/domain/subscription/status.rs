#[derive(Debug, Clone)]
pub enum Status {
    Open,
    Closed,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Open => "open".to_owned(),
            Status::Closed => "closed".to_owned(),
        }
    }
}
