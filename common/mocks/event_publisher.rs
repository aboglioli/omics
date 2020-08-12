use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::event::{Event, EventPublisher};
use crate::result::Result;

pub struct FakeEventPublisher {
    events: Mutex<Vec<Event>>,
}

impl FakeEventPublisher {
    pub fn new() -> Self {
        FakeEventPublisher {
            events: Mutex::new(Vec::new()),
        }
    }

    pub async fn events(&self) -> Vec<Event> {
        self.events.lock().await.clone()
    }
}

#[async_trait]
impl EventPublisher for FakeEventPublisher {
    type Output = bool;

    async fn publish(&self, event: Event) -> Result<Self::Output> {
        self.events.lock().await.push(event);
        Ok(true)
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output> {
        self.events.lock().await.extend(events);
        Ok(true)
    }
}
