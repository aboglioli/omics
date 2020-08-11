use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::domain::event::serializer;

#[derive(Serialize, Deserialize, Debug)]
pub enum CollectionEvent {
    Created {
        id: String,
        name: String,
        synopsis: String,
    },
}

impl ToString for CollectionEvent {
    fn to_string(&self) -> String {
        match self {
            CollectionEvent::Created { .. } => "created".to_owned(),
        }
    }
}

impl ToEvent for CollectionEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = serializer::serialize(&self, "collection")?;

        Ok(Event::new("collection", &self.to_string(), payload))
    }
}
