use std::fmt::Debug;

use crate::error::Error;

pub trait Event: Debug {
    fn code(&self) -> &str;
    fn payload(&self) -> Vec<u8>;
}

pub trait EventPublisher {
    type Output;

    fn publish(&self, topic: &str, event: &dyn Event) -> Result<Self::Output, Error>;
}

pub type Subscription<'a> = Box<dyn FnMut(&dyn Event) -> Result<(), Error> + 'a>;

pub trait EventSubscriber<'a> {
    type Output;

    fn subscribe(&self, topic: &str, cb: Subscription<'a>) -> Result<Self::Output, Error>;
}
