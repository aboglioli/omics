use std::fmt::Debug;

use crate::error::Error;

pub trait Event: Debug {
    fn code(&self) -> &str;
    fn payload(&self) -> Vec<u8>;
}

// pub trait EventPublisher {
//     fn publish<E: Event + 'static>(&self, topic: &str, event: E) -> Result<(), Error>;
// }

pub trait EventPublisher {
    fn publish(&self, topic: &str, event: Box<dyn Event>) -> Result<(), Error>;
}
