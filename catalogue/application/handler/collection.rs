use std::sync::Arc;

use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::CollectionEvent;

use crate::domain::catalogue::{CatalogueRepository, CollectionService};

pub struct CollectionHandler {
    catalogue_repo: Arc<dyn CatalogueRepository>,

    collection_serv: Arc<dyn CollectionService>,
}

impl CollectionHandler {
    pub fn new(
        catalogue_repo: Arc<dyn CatalogueRepository>,
        collection_serv: Arc<dyn CollectionService>,
    ) -> Self {
        CollectionHandler {
            catalogue_repo,
            collection_serv,
        }
    }
}

#[async_trait]
impl EventHandler for CollectionHandler {
    fn topic(&self) -> &str {
        "collection"
    }

    async fn handle(&mut self, event: &Event) -> Result<bool> {
        let event = match serde_json::from_slice(event.payload()) {
            Ok(event) => event,
            Err(err) => return Err(Error::new("handler", "deserialize").wrap_raw(err).build()),
        };

        let mut catalogue = self.catalogue_repo.find().await?;

        match event {
            CollectionEvent::Created { id, .. } => {
                let collection = self.collection_serv.get_by_id(&id).await?;
                catalogue.add_collection(collection);
            }
            CollectionEvent::Deleted { id } => {
                catalogue.remove_collection(&id);
            }
            CollectionEvent::HeaderUpdated { id, .. } => {
                let collection = self.collection_serv.get_by_id(&id).await?;
                catalogue.add_collection(collection);
            }
            CollectionEvent::PublicationAdded { id, .. } => {
                let collection = self.collection_serv.get_by_id(&id).await?;
                catalogue.add_collection(collection);
            }
            CollectionEvent::PublicationRemoved { id, .. } => {
                let collection = self.collection_serv.get_by_id(&id).await?;
                catalogue.add_collection(collection);
            } // _ => return Ok(false),
        }

        self.catalogue_repo.save(&mut catalogue).await?;

        Ok(true)
    }
}
