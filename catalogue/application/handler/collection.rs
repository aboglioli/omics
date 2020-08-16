use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use domain::event::CollectionEvent;

use crate::domain::catalogue::{CatalogueRepository, CollectionService};

pub struct CollectionHandler<'a, CRepo, CServ> {
    catalogue_repo: &'a CRepo,

    collection_serv: &'a CServ,
}

impl<'a, CRepo, CServ> CollectionHandler<'a, CRepo, CServ> {
    pub fn new(catalogue_repo: &'a CRepo, collection_serv: &'a CServ) -> Self {
        CollectionHandler {
            catalogue_repo,
            collection_serv,
        }
    }
}

#[async_trait]
impl<'a, CRepo, CServ> EventHandler for CollectionHandler<'a, CRepo, CServ>
where
    CRepo: CatalogueRepository + Sync + Send,
    CServ: CollectionService + Sync + Send,
{
    type Output = bool;

    fn topic(&self) -> &str {
        "collection"
    }

    async fn handle(&mut self, event: &Event) -> Result<Self::Output> {
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
