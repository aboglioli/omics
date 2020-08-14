use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::publication::PublicationId;

#[derive(Debug, Clone)]
pub struct Item {
    publication_id: PublicationId,
    date: DateTime<Utc>,
}

impl Item {
    pub fn new(publication_id: PublicationId) -> Result<Self> {
        Ok(Item {
            publication_id,
            date: Utc::now(),
        })
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
