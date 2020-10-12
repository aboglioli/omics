use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum CollectionEvent {
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
    PublicationAdded {
        id: String,
        publication_id: String,
    },
    PublicationRemoved {
        id: String,
        publication_id: String,
    },
    Deleted {
        id: String,
    },
}

impl ToString for CollectionEvent {
    fn to_string(&self) -> String {
        match self {
            CollectionEvent::Created { .. } => "created".to_owned(),
            CollectionEvent::HeaderUpdated { .. } => "header-updated".to_owned(),
            CollectionEvent::PublicationAdded { .. } => "publication-added".to_owned(),
            CollectionEvent::PublicationRemoved { .. } => "publication-removed".to_owned(),
            CollectionEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for CollectionEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new(
            "collection".to_owned(),
            self.to_string(),
            serde_json::to_value(&self)?,
        ))
    }
}
