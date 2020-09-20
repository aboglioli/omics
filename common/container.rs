use async_trait::async_trait;

use crate::event::EventSubscriber;
use crate::result::Result;

#[async_trait]
pub trait Container: Sync + Send {
    async fn start(&self) -> Result<()> {
        Ok(())
    }

    async fn subscribe<ES>(&self, _event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber + Sync + Send,
    {
        Ok(())
    }

    async fn populate(&self) -> Result<()> {
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
