use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ContractEvent {
    Requested {
        id: String,
        publication_id: String,
        author_id: String,
        timestamp: DateTime<Utc>,
    },
    Approved {
        id: String,
        publication_id: String,
        author_id: String,
        content_manager_id: String,
        timestamp: DateTime<Utc>,
    },
    Rejected {
        id: String,
        publication_id: String,
        author_id: String,
        content_manager_id: String,
        timestamp: DateTime<Utc>,
    },
    Cancelled {
        id: String,
        publication_id: String,
        author_id: String,
        timestamp: DateTime<Utc>,
    },
}
