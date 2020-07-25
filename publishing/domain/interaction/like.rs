use chrono::{DateTime, Utc};

use common::error::Error;

use crate::domain::publication::{Publication, PublicationID};
use crate::domain::reader::{Reader, ReaderID};

pub struct Like {
    reader_id: ReaderID,
    publication_id: PublicationID,
    date: DateTime<Utc>,
}

impl Like {
    pub fn new(reader_id: ReaderID, publication_id: PublicationID) -> Result<Like, Error> {
        Ok(Like {
            reader_id,
            publication_id,
            date: Utc::now(),
        })
    }
}
