use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::SubscriptionEvent;

use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct SubscriptionHandler {
    reader_repo: Arc<dyn ReaderRepository>,
}

impl SubscriptionHandler {
    pub fn new(reader_repo: Arc<dyn ReaderRepository>) -> Self {
        SubscriptionHandler { reader_repo }
    }
}

#[async_trait]
impl EventHandler for SubscriptionHandler {
    fn topic(&self) -> &str {
        "subscription"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: SubscriptionEvent = serde_json::from_slice(event.payload())?;

        match event {
            SubscriptionEvent::PaymentAdded { user_id, .. } => {
                let mut reader = self
                    .reader_repo
                    .find_by_id(&ReaderId::new(user_id)?)
                    .await?;
                reader.subscribe()?;
                self.reader_repo.save(&mut reader).await?;
            }
            SubscriptionEvent::PaymentRequired { user_id, .. }
            | SubscriptionEvent::Disabled { user_id, .. } => {
                let mut reader = self
                    .reader_repo
                    .find_by_id(&ReaderId::new(user_id)?)
                    .await?;
                reader.unsubscribe()?;
                self.reader_repo.save(&mut reader).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
