use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::publication::{PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct InteractionHandler {
    reader_repo: Arc<dyn ReaderRepository>,
    publication_repo: Arc<dyn PublicationRepository>,
}

impl InteractionHandler {
    pub fn new(
        reader_repo: Arc<dyn ReaderRepository>,
        publication_repo: Arc<dyn PublicationRepository>,
    ) -> Self {
        InteractionHandler {
            reader_repo,
            publication_repo,
        }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    fn topic(&self) -> &str {
        "publication"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: PublicationEvent = serde_json::from_slice(event.payload()).map_err(|err| {
            Error::new("interaction_handler", "deserialize")
                .wrap_raw(err)
                .build()
        })?;

        match event {
            PublicationEvent::Read {
                reader_id,
                publication_id,
            } => {
                let mut reader = self
                    .reader_repo
                    .find_by_id(&ReaderId::new(reader_id)?)
                    .await?;
                let publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;

                reader.preferences_mut().add_publication(&publication)?;

                self.reader_repo.save(&mut reader).await?;

                return Ok(true);
            }
            _ => {}
        }

        Ok(false)
    }
}
