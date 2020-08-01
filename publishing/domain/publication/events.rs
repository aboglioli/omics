use common::error::Error;
use common::event::{Event, ToEvent};

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{PageNumber, PublicationId, Tag};

pub enum PublicationEvent {
    PublicationCreated {
        id: PublicationId,
        name: String,
        synopsis: String,
        author_id: AuthorId,
        pages_count: PageNumber,
        category_id: CategoryId,
        tags: Vec<Tag>,
    },
    PublicationApproved {
        id: PublicationId,
    },
    PublicationRejected {
        id: PublicationId,
    },
    PublicationPublished {
        id: PublicationId,
    },
}

impl ToEvent for PublicationEvent {
    fn to_event(&self) -> Result<Event, Error> {
        Ok(Event::new("", "", Vec::new()))
    }
}
