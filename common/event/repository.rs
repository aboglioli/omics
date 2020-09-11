use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::event::{Event, EventId};
use crate::result::Result;

#[async_trait]
pub trait EventRepository: Sync + Send {
    async fn find_all(&self) -> Result<Vec<Event>>;
    async fn find_after_id(&self, id: &EventId) -> Result<Vec<Event>>;
    async fn find_from_date(
        &self,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Event>>;

    async fn save(&self, event: &Event) -> Result<()>;
}
