use common::event::{Event, ToEvent};
use common::result::Result;

use crate::domain::user::UserId;

#[derive(Debug)]
pub enum UserEvent {
    Updated {
        id: UserId,
        name: String,
        lastname: String,
    },
    Registered {
        id: UserId,
        username: String,
        email: String,
    },
}

impl ToEvent for UserEvent {
    fn to_event(&self) -> Result<Event> {
        Ok(Event::new("", "", Vec::new()))
    }
}
