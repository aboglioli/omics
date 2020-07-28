use crate::error::Error;
use crate::event::Event;

pub trait EventPublisher {
    type Output;

    fn publish(&self, event: Event) -> Result<Self::Output, Error>;

    fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output, Error>;
}
