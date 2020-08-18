use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;

use crate::infrastructure::event::EventRepository;

pub struct EventLogger {
    event_repo: Arc<EventRepository>,
}

impl EventLogger {
    pub fn new(event_repo: Arc<EventRepository>) -> Self {
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
        let payload = String::from_utf8_lossy(event.payload());

        println!("# EVENT");
        println!("- topic: {}", event.topic());
        println!("- code: {}", event.code());
        println!("- payload: {:?}", payload);

        self.event_repo.add(event.clone()).await;

        Ok(true)
    }
}
