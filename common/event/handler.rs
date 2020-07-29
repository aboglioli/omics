use async_trait::async_trait;

use crate::error::Error;
use crate::event::Event;

#[async_trait]
pub trait EventHandler: Send {
    type Output;

    fn topic(&self) -> &str;

    async fn handle(&mut self, event: &Event) -> Result<Self::Output, Error>;
}
