use chrono::{DateTime, Utc};

use common::error::Error;

use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct Like {
    reader_id: ReaderId,
    publication_id: PublicationId,
    date: DateTime<Utc>,
}

impl Like {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId) -> Result<Like, Error> {
        Ok(Like {
            reader_id,
            publication_id,
            date: Utc::now(),
        })
    }
}
