use async_trait::async_trait;

use crate::event::EventHandler;
use crate::result::Result;

#[async_trait]
pub trait EventSubscriber {
    type Output;

    async fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output> + Sync>,
    ) -> Result<Self::Output>;
}
