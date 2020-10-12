use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::UserEvent;

use crate::domain::reader::{Reader, ReaderId, ReaderRepository};

pub struct ReaderFromUserHandler {
    reader_repo: Arc<dyn ReaderRepository>,
}

impl ReaderFromUserHandler {
    pub fn new(reader_repo: Arc<dyn ReaderRepository>) -> Self {
        ReaderFromUserHandler { reader_repo }
    }
}

#[async_trait]
impl EventHandler for ReaderFromUserHandler {
    fn topic(&self) -> &str {
        "user"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: UserEvent = serde_json::from_value(event.payload())
            .map_err(|err| Error::new("reader_from_user_handler", "deserialize").wrap_raw(err))?;

        match event {
            UserEvent::Validated { id } => {
                let mut reader = Reader::new(ReaderId::new(id)?)?;
                self.reader_repo.save(&mut reader).await?;
            }
            UserEvent::Deleted { id } => {
                let mut reader = self.reader_repo.find_by_id(&ReaderId::new(id)?).await?;
                reader.delete()?;
                self.reader_repo.save(&mut reader).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
