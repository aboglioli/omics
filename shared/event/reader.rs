use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum ReaderEvent {
    PublicationAddedToFavorites {
        reader_id: String,
        publication_id: String,
    },
    PublicationRemovedFromFavorites {
        reader_id: String,
        publication_id: String,
    },
    CollectionAddedToFavorites {
        reader_id: String,
        collection_id: String,
    },
    CollectionRemovedFromFavorites {
        reader_id: String,
        collection_id: String,
    },
}

impl ToString for ReaderEvent {
    fn to_string(&self) -> String {
        match self {
            ReaderEvent::PublicationAddedToFavorites { .. } => {
                "publication-added-to-favorites".to_owned()
            }
            ReaderEvent::PublicationRemovedFromFavorites { .. } => {
                "publication-removed-from-favorites".to_owned()
            }
            ReaderEvent::CollectionAddedToFavorites { .. } => {
                "collection-added-to-favorites".to_owned()
            }
            ReaderEvent::CollectionRemovedFromFavorites { .. } => {
                "collection-removed-from-favorites".to_owned()
            }
        }
    }
}

impl ToEvent for ReaderEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "reader".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
