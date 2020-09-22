use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::event::{Event, EventId};
use crate::result::Result;

#[async_trait]
pub trait EventRepository: Sync + Send {
    async fn search(
        &self,
        after_id: Option<&EventId>,
        topic: Option<&String>,
        code: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Event>>;

    async fn save(&self, event: &Event) -> Result<()>;
}
