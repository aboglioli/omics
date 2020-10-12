use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use common::event::ApplyEvent;
use common::result::Result;
use shared::event::UserEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Users {
    new: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    users: Users,

    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

impl Report {
    pub fn new(from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Self> {
        Ok(Report {
            users: Users { new: 0 },

            from,
            to,
        })
    }
}

impl ApplyEvent<UserEvent> for Report {
    fn apply(&mut self, _event: &UserEvent) -> Result<()> {
        Ok(())
    }
}
