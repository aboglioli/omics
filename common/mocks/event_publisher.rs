use async_trait::async_trait;
use tokio::sync::oneshot::{self, Receiver};
use tokio::sync::Mutex;

use crate::event::{Event, EventPublisher, PublicationResult};
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
    async fn publish(&self, event: Event) -> Result<Receiver<PublicationResult>> {
        self.events.lock().await.push(event);
        let (_, tx) = oneshot::channel();
        Ok(tx)
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Receiver<PublicationResult>> {
        self.events.lock().await.extend(events);
        let (_, tx) = oneshot::channel();
        Ok(tx)
    }
}
