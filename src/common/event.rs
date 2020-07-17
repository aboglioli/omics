use crate::common::error::Error;

pub trait Event {
    fn code(&self) -> &str;
    fn payload(&self) -> Vec<u8>;
}

pub trait EventPublisher {
    fn publish<E: Event + 'static>(&self, topic: &str, event: E) -> Result<(), Error>;
}

#[cfg(test)]
mod event_test;
