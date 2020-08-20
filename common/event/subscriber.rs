use async_trait::async_trait;

use crate::event::EventHandler;
use crate::result::Result;

#[async_trait]
pub trait EventSubscriber {
    async fn subscribe(
        &self,
        handler: Box<dyn EventHandler>, // TODO: use generics.
    ) -> Result<bool>;
}
