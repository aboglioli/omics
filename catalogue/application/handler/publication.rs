use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::catalogue::{CatalogueRepository, PublicationService};

pub struct PublicationHandler {
    catalogue_repo: Arc<dyn CatalogueRepository>,

    publication_serv: Arc<dyn PublicationService>,
}

impl PublicationHandler {
    pub fn new(
        catalogue_repo: Arc<dyn CatalogueRepository>,
        publication_serv: Arc<dyn PublicationService>,
    ) -> Self {
        PublicationHandler {
            catalogue_repo,
            publication_serv,
        }
    }
}

#[async_trait]
impl EventHandler for PublicationHandler {
    fn topic(&self) -> &str {
        "publication"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event = match serde_json::from_slice(event.payload()) {
            Ok(event) => event,
            Err(err) => return Err(Error::new("handler", "deserialize").wrap_raw(err).build()),
        };

        let mut catalogue = self.catalogue_repo.find().await?;

        match event {
            PublicationEvent::Published { id } => {
                let publication = self.publication_serv.get_by_id(&id).await?;
                catalogue.add_publication(publication);
            }
            PublicationEvent::Deleted { id } => {
                catalogue.remove_publication(&id);
            }
            PublicationEvent::ChangedToDraft { id } => {
                catalogue.remove_publication(&id);
            }
            PublicationEvent::StatisticsUpdated { id, .. } => {
                let publication = self.publication_serv.get_by_id(&id).await?;
                catalogue.add_publication(publication);
            }
            _ => return Ok(false),
        }

        self.catalogue_repo.save(&mut catalogue).await?;

        Ok(true)
    }
}
