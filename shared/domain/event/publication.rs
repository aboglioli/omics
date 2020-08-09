use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::domain::event::serializer;

#[derive(Serialize, Deserialize, Debug)]
pub enum PublicationEvent {
    Created {
        id: String,
        name: String,
        synopsis: String,
        author_id: String,
        pages_count: u32,
        category_id: String,
        tags: Vec<String>,
    },
    Published {
        id: String,
    },
    Approved {
        id: String,
    },
    Rejected {
        id: String,
    },
    Deleted {
        id: String,
    },
    Viewed {
        reader_id: String,
        publication_id: String,
    },
    Read {
        reader_id: String,
        publication_id: String,
    },
    Liked {
        reader_id: String,
        publication_id: String,
    },
    Unliked {
        reader_id: String,
        publication_id: String,
    },
    Reviewed {
        reader_id: String,
        publication_id: String,
        stars: u8,
    },
    ReviewDeleted {
        reader_id: String,
        publication_id: String,
    },
}

impl ToString for PublicationEvent {
    fn to_string(&self) -> String {
        match self {
            PublicationEvent::Created { .. } => "created".to_owned(),
            PublicationEvent::Published { .. } => "published".to_owned(),
            PublicationEvent::Approved { .. } => "approved".to_owned(),
            PublicationEvent::Rejected { .. } => "rejected".to_owned(),
            PublicationEvent::Deleted { .. } => "deleted".to_owned(),
            PublicationEvent::Viewed { .. } => "viewed".to_owned(),
            PublicationEvent::Read { .. } => "read".to_owned(),
            PublicationEvent::Liked { .. } => "liked".to_owned(),
            PublicationEvent::Unliked { .. } => "unliked".to_owned(),
            PublicationEvent::Reviewed { .. } => "reviewed".to_owned(),
            PublicationEvent::ReviewDeleted { .. } => "review-deleted".to_owned(),
        }
    }
}

impl ToEvent for PublicationEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = serializer::serialize(&self, "publication")?;

        Ok(Event::new("publication", &self.to_string(), payload))
    }
}
