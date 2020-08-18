use async_trait::async_trait;
use tokio::sync::oneshot::Receiver;

use crate::event::Event;
use crate::result::Result;

#[derive(Default)]
pub struct PublicationResult {
    pub published_events: u32,
    pub ok_handlers: u32,
    pub err_handlers: u32,
}

impl PublicationResult {
    pub fn published_events(&self) -> u32 {
        self.published_events
    }

    pub fn ok_handlers(&self) -> u32 {
        self.ok_handlers
    }

    pub fn err_handlers(&self) -> u32 {
        self.err_handlers
    }

    pub fn activated_handlers(&self) -> u32 {
        self.ok_handlers + self.err_handlers
    }
}

#[async_trait]
pub trait EventPublisher: Sync + Send {
    async fn publish(&self, event: Event) -> Result<Receiver<PublicationResult>>;

    async fn publish_all(&self, events: Vec<Event>) -> Result<Receiver<PublicationResult>>;
}
