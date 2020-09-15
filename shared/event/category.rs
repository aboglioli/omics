use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub enum CategoryEvent {
    Created { id: String, name: String },
    Updated { id: String, name: String },
    Deleted { id: String },
}

impl ToString for CategoryEvent {
    fn to_string(&self) -> String {
        match self {
            CategoryEvent::Created { .. } => "created".to_owned(),
            CategoryEvent::Updated { .. } => "updated".to_owned(),
            CategoryEvent::Deleted { .. } => "deleted".to_owned(),
        }
    }
}

impl ToEvent for CategoryEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = util::serialize(&self, "category")?;

        Ok(Event::new("category".to_owned(), self.to_string(), payload))
    }
}
