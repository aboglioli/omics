use std::fmt::Debug;

use chrono::{DateTime, Utc};

use crate::error::Error;

pub type EventPayload = Vec<u8>;

#[derive(Debug)]
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

    pub fn payload(self) -> EventPayload {
        self.payload
    }
}

pub trait EventPublisher {
    type Output;

    fn publish(&self, event: Event) -> Result<Self::Output, Error>;

    fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output, Error>;
}

pub trait EventSubscriber {
    type Output;

    fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output>>,
    ) -> Result<Self::Output, Error>;
}

pub trait EventHandler: Send {
    type Output;

    fn topic(&self) -> &str;

    fn handle(&mut self, event: &Event) -> Result<Self::Output, Error>;
}
