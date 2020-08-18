use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;

pub struct EventLogger {
    events: Vec<Event>,
}

impl EventLogger {
    pub fn new() -> Self {
        println!("[DEV] EventLogger added");
        EventLogger { events: Vec::new() }
    }
}

#[async_trait]
impl EventHandler for EventLogger {
    type Output = bool;

    fn topic(&self) -> &str {
        ".*"
    }

    async fn handle(&mut self, event: &Event) -> Result<Self::Output> {
        let payload = String::from_utf8_lossy(event.payload());

        println!("# EVENT");
        println!("- topic: {}", event.topic());
        println!("- code: {}", event.code());
        println!("- payload: {:?}", payload);

        self.events.push(event.clone());

        Ok(true)
    }
}
