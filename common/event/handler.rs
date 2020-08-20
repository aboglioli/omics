use async_trait::async_trait;

use crate::event::Event;
use crate::result::Result;

#[async_trait]
pub trait EventHandler: Sync + Send {
    fn topic(&self) -> &str;

    async fn handle(&mut self, event: &Event) -> Result<bool>;
}
