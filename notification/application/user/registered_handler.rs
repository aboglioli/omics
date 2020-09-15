use async_trait::async_trait;

use common::event::{Event, EventHandler};

pub struct RegisteredHandler {
}

#[async_trait]
impl EventHandler for RegisteredHandler {
    fn topic() -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        Ok(true)
    }
}
