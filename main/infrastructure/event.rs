use common::tokio::sync::Mutex;
// use tokio::sync::Mutex;

use common::event::Event;

pub struct EventRepository {
    events: Mutex<Vec<Event>>,
}

impl EventRepository {
    pub fn new() -> Self {
        EventRepository {
            events: Mutex::new(Vec::new()),
        }
    }

    pub async fn add(&self, event: Event) {
        self.events.lock().await.push(event);
    }

    pub async fn events(&self) -> Vec<Event> {
        self.events.lock().await.clone()
    }
}
