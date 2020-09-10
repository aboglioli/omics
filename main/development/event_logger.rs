use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler, EventRepository};
use common::result::Result;

pub struct EventLogger {
    event_repo: Arc<dyn EventRepository>,
}

impl EventLogger {
    pub fn new(event_repo: Arc<dyn EventRepository>) -> Self {
        println!("[DEV] EventLogger added");
        EventLogger { event_repo }
    }
}

#[async_trait]
impl EventHandler for EventLogger {
    fn topic(&self) -> &str {
        ".*"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        self.event_repo.save(&event).await?;
        Ok(true)
    }
}
