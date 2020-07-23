use chrono::{DateTime, Utc};

use common::error::Error;
use common::model::{Entity, ID};

use crate::domain::publication::{Publication, PublicationID};
use crate::domain::reader::{Reader, ReaderID};

pub struct Like {
    reader_id: ReaderID,
    publication_id: PublicationID,
    date: DateTime<Utc>,
}

impl Like {
    pub fn new(reader: &Reader, publication: &Publication) -> Result<Like, Error> {
        Ok(Like {
            reader_id: reader.id().value(),
            publication_id: publication.id().value(),
            date: Utc::now(),
        })
    }
}
