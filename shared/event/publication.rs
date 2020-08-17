use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub enum PublicationEvent {
    Created {
        id: String,
        author_id: String,
        name: String,
        synopsis: String,
        category_id: String,
        tags: Vec<String>,
        cover: String,
    },
    HeaderUpdated {
        id: String,
        name: String,
        synopsis: String,
        category_id: String,
        tags: Vec<String>,
        cover: String,
    },
    PagesUpdated {
        id: String,
        pages_count: usize,
    },
    ChangedToDraft {
        id: String,
    },
    ApprovalWaited {
        id: String,
    },
    Published {
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
        unique: bool,
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
        comment: String,
    },
    ReviewDeleted {
        reader_id: String,
        publication_id: String,
    },
    StatisticsUpdated {
        id: String,
        views: u32,
        unique_views: u32,
        readings: u32,
        likes: u32,
        reviews: u32,
        stars: f32,
    },
    ContractAdded {
        id: String,
    },
    ContractRemoved {
        id: String,
    },
}

impl ToString for PublicationEvent {
    fn to_string(&self) -> String {
        match self {
            PublicationEvent::Created { .. } => "created".to_owned(),
            PublicationEvent::HeaderUpdated { .. } => "header-updated".to_owned(),
            PublicationEvent::PagesUpdated { .. } => "pages-updated".to_owned(),
            PublicationEvent::ChangedToDraft { .. } => "changed-to-draft".to_owned(),
            PublicationEvent::ApprovalWaited { .. } => "approval-waited".to_owned(),
            PublicationEvent::Published { .. } => "published".to_owned(),
            PublicationEvent::Rejected { .. } => "rejected".to_owned(),
            PublicationEvent::Deleted { .. } => "deleted".to_owned(),
            PublicationEvent::Viewed { .. } => "viewed".to_owned(),
            PublicationEvent::Read { .. } => "read".to_owned(),
            PublicationEvent::Liked { .. } => "liked".to_owned(),
            PublicationEvent::Unliked { .. } => "unliked".to_owned(),
            PublicationEvent::Reviewed { .. } => "reviewed".to_owned(),
            PublicationEvent::ReviewDeleted { .. } => "review-deleted".to_owned(),
            PublicationEvent::StatisticsUpdated { .. } => "statistics-updated".to_owned(),
            PublicationEvent::ContractAdded { .. } => "contract-added".to_owned(),
            PublicationEvent::ContractRemoved { .. } => "contract-removed".to_owned(),
        }
    }
}

impl ToEvent for PublicationEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = util::serialize(&self, "publication")?;

        Ok(Event::new("publication", &self.to_string(), payload))
    }
}
