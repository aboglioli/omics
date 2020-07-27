use std::fmt::Debug;

use crate::error::Error;

pub trait Event: Debug {
    fn code(&self) -> &str;
    fn payload(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct EventWithTopic {
    topic: String,
    event: Box<dyn Event>,
}

impl EventWithTopic {
    pub fn new<E: Event + 'static>(topic: &str, event: E) -> Self {
        EventWithTopic {
            topic: topic.to_owned(),
            event: Box::new(event),
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn event(&self) -> &dyn Event {
        self.event.as_ref()
    }
}

pub trait EventPublisher {
    type Output;

    fn publish(&self, topic: &str, event: &dyn Event) -> Result<Self::Output, Error>;

    fn publish_all(&self, events_with_topic: &[EventWithTopic]) -> Result<Self::Output, Error>;
}

pub type Subscription<'a> = Box<dyn FnMut(&str, &dyn Event) -> Result<(), Error> + 'a>;

pub trait EventSubscriber<'a> {
    type Output;

    fn subscribe(&self, topic: &str, cb: Subscription<'a>) -> Result<Self::Output, Error>;
}
