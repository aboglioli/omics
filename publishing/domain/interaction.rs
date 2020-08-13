mod kind;
mod repository;
mod stars;
pub use kind::*;
pub use repository::*;
pub use stars::*;

use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[derive(Debug, Clone)]
pub struct Interaction {
    reader_id: ReaderId,
    publication_id: PublicationId,
    kind: Kind,
    date: DateTime<Utc>,
}

impl Interaction {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId, kind: Kind) -> Result<Self> {
        Ok(Interaction {
            reader_id,
            publication_id,
            kind,
            date: Utc::now(),
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
