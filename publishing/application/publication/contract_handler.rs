use std::sync::Arc;

use async_trait::async_trait;

use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::ContractEvent;

use crate::domain::publication::{PublicationId, PublicationRepository};

pub struct ContractHandler {
    publication_repo: Arc<dyn PublicationRepository>,
}

impl ContractHandler {
    pub fn new(publication_repo: Arc<dyn PublicationRepository>) -> Self {
        ContractHandler { publication_repo }
    }
}

#[async_trait]
impl EventHandler for ContractHandler {
    fn topic(&self) -> &str {
        "contract"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event: ContractEvent = serde_json::from_value(event.payload())?;

        match event {
            ContractEvent::Approved { publication_id, .. } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;
                publication.add_contract()?;
                self.publication_repo.save(&mut publication).await?;
            }
            ContractEvent::Cancelled { publication_id, .. } => {
                let mut publication = self
                    .publication_repo
                    .find_by_id(&PublicationId::new(publication_id)?)
                    .await?;
                publication.remove_contract()?;
                self.publication_repo.save(&mut publication).await?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
