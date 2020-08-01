use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::interaction::Stars;
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct Review {
    reader_id: ReaderId,
    publication_id: PublicationId,
    stars: Stars,
    date: DateTime<Utc>,
}

impl Review {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId, stars: Stars) -> Result<Review> {
        Ok(Review {
            reader_id,
            publication_id,
            stars,
            date: Utc::now(),
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn stars(&self) -> &Stars {
        &self.stars
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
