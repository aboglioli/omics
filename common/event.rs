mod handler;
pub mod inmem;
mod publisher;
mod subscriber;
pub use handler::*;
pub use publisher::*;
pub use subscriber::*;

use std::fmt::Debug;

use chrono::{DateTime, Utc};

use crate::result::Result;

#[derive(Debug, Clone)]
pub struct Event {
    topic: String,
    code: String,
    timestamp: DateTime<Utc>,
    payload: Vec<u8>,
}

impl Event {
    pub fn new<S: Into<String>>(topic: S, code: S, payload: Vec<u8>) -> Self {
        Event {
            topic: topic.into(),
            code: code.into(),
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

    pub fn payload(&self) -> &[u8] {
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

pub trait ApplyEvent<E> {
    fn apply(&self, event: E) -> Result<()>;

    fn apply_all(&self, events: Vec<E>) -> Result<()> {
        for event in events.into_iter() {
            self.apply(event)?;
        }
        Ok(())
    }
}
