use async_trait::async_trait;

use crate::error::Error;
use crate::event::Event;

#[async_trait]
pub trait EventPublisher {
    type Output;

    async fn publish(&self, event: Event) -> Result<Self::Output, Error>;

    async fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output, Error>;
}
