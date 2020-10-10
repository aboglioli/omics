use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    new_users: Option<u64>,

    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Report {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Self> {
        Ok(Report {
            new_users: None,

            from,
            to,
        })
    }
}
