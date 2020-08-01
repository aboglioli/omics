use common::event::{Event, ToEvent};
use common::result::Result;

use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[derive(Debug)]
pub enum ReaderEvent {
    PublicationRead {
        reader_id: ReaderId,
        publication_id: PublicationId,
    },
    PublicationLiked {
        reader_id: ReaderId,
        publication_id: PublicationId,
    },
    PublicationUnliked {
        reader_id: ReaderId,
        publication_id: PublicationId,
    },
    PublicationReviewed {
        reader_id: ReaderId,
        publication_id: PublicationId,
    },
    PublicationUnreviewed {
        reader_id: ReaderId,
        publication_id: PublicationId,
    },
}

impl ToEvent for ReaderEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new("", "", Vec::new()))
    }
}
