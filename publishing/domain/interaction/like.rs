use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct Like {
    reader_id: ReaderId,
    publication_id: PublicationId,
    date: DateTime<Utc>,
}

impl Like {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId) -> Result<Like> {
        Ok(Like {
            reader_id,
            publication_id,
            date: Utc::now(),
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
