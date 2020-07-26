use std::fmt::Debug;

use crate::error::Error;

pub trait Event: Debug {
    fn code(&self) -> &str;
    fn payload(&self) -> Vec<u8>;
}

pub trait EventPublisher {
    fn publish(&self, topic: &str, event: Box<dyn Event>) -> Result<(), Error>;
}

pub type Subscription = Box<dyn FnMut(&dyn Event) -> Result<(), Error>>;

pub trait EventSubscriber {
    fn subscribe(&self, topic: &str, cb: Subscription) -> Result<(), Error>;
}
