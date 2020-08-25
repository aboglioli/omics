use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::reader::ReaderId;

#[derive(Debug, Clone)]
pub struct Follow {
    reader_id: ReaderId,
    author_id: AuthorId,
    date: DateTime<Utc>,
}

impl Follow {
    pub fn new(reader_id: ReaderId, author_id: AuthorId) -> Result<Self> {
        Ok(Follow {
            reader_id,
            author_id,
            date: Utc::now(),
        })
    }

    pub fn reader_id(&self) -> &ReaderId {
        &self.reader_id
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
