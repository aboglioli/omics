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

    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }
}

pub trait ToEvent {
    fn to_event(&self) -> Result<Event, Error>;
}

#[derive(Debug)]
pub struct BasicEvent {
    topic: String,
    code: String,
}

impl ToEvent for BasicEvent {
    fn to_event(&self) -> Result<Event, Error> {
        Ok(Event::new(&self.topic, &self.code, Vec::new()))
    }
}
