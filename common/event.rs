mod handler;
mod inmem;
mod publisher;
mod subscriber;
pub use handler::*;
pub use inmem::*;
pub use publisher::*;
pub use subscriber::*;

use std::fmt::Debug;

use chrono::{DateTime, Utc};

use crate::result::Result;

pub type EventPayload = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Event {
    topic: String,
    code: String,
    timestamp: DateTime<Utc>,
    payload: EventPayload,
}

impl Event {
    pub fn new(topic: &str, code: &str, payload: EventPayload) -> Self {
        Event {
            topic: topic.to_owned(),
            code: code.to_owned(),
            timestamp: Utc::now(),
            payload,
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }
}

pub trait ToEvent {
    fn to_event(&self) -> Result<Event>;
}

impl ToEvent for Event {
    fn to_event(&self) -> Result<Event> {
        Ok(self.clone())
    }
}
