use serde::{Deserialize, Serialize};

use common::event::{Event, ToEvent};
use common::result::Result;

use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthorEvent {
    Followed {
        author_id: String,
        reader_id: String,
    },
}

impl ToString for AuthorEvent {
    fn to_string(&self) -> String {
        match self {
            AuthorEvent::Followed { .. } => "followed".to_owned(),
        }
    }
}

impl ToEvent for AuthorEvent {
    fn to_event(&self) -> Result<Event> {
        let payload = util::serialize(&self, "author")?;

        Ok(Event::new("author".to_owned(), self.to_string(), payload))
    }
}
