mod handler;
mod publisher;
mod repository;
mod subscriber;
pub use handler::*;
pub use publisher::*;
pub use repository::*;
pub use subscriber::*;

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::model::StringId;
use crate::result::Result;

pub type EventId = StringId;

#[derive(Debug, Clone)]
pub struct Event {
    id: EventId,
    topic: String,
    code: String,
    timestamp: DateTime<Utc>,
    payload: Value,
}

impl Event {
    pub fn new<S: Into<String>>(topic: S, code: S, payload: Value) -> Self {
        Event {
            id: EventId::new(Uuid::new_v4().to_string()).unwrap(),
            topic: topic.into(),
            code: code.into(),
            timestamp: Utc::now(),
            payload,
        }
    }

    pub fn build(
        id: EventId,
        topic: String,
        code: String,
        timestamp: DateTime<Utc>,
        payload: Value,
    ) -> Self {
        Event {
            id,
            topic,
            code,
            timestamp,
            payload,
        }
    }

    pub fn id(&self) -> &EventId {
        &self.id
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

    pub fn payload(&self) -> Value {
        self.payload.clone()
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
    fn apply(&mut self, event: &E) -> Result<()>;

    fn apply_all(&mut self, events: Vec<&E>) -> Result<()> {
        for event in events.into_iter() {
            self.apply(event)?;
        }
        Ok(())
    }
}
