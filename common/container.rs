use async_trait::async_trait;

use crate::event::EventSubscriber;
use crate::result::Result;

#[async_trait]
pub trait Container: Sync + Send {
    async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber + Sync + Send;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
}
