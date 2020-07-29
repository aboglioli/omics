use async_trait::async_trait;

use crate::error::Error;
use crate::event::EventHandler;

#[async_trait]
pub trait EventSubscriber {
    type Output;

    async fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output> + Sync>,
    ) -> Result<Self::Output, Error>;
}
