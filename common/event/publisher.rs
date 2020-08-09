use async_trait::async_trait;

use crate::event::Event;
use crate::result::Result;

#[async_trait]
pub trait EventPublisher {
    type Output;

    async fn publish(&self, event: Event) -> Result<Self::Output>;

    // TODO: make main function
    async fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output>;
}
