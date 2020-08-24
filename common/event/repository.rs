use async_trait::async_trait;

use crate::event::{Event, EventId};
use crate::result::Result;

#[async_trait]
pub trait EventRepository: Sync + Send {
    async fn find_all(&self) -> Result<Vec<Event>>;
    async fn find_from(&self, id: &EventId) -> Result<Vec<Event>>;

    async fn save(&self, event: &Event) -> Result<()>;
}
