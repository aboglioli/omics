use std::sync::Arc;

use common::event::{EventPublisher, EventSubscriber};
use common::result::Result;

use crate::application::handler::{CollectionHandler, PublicationHandler};
use crate::domain::catalogue::{CatalogueRepository, CollectionService, PublicationService};

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    catalogue_repo: Arc<dyn CatalogueRepository>,

    collection_serv: Arc<dyn CollectionService>,
    publication_serv: Arc<dyn PublicationService>,
}

impl<EPub> Container<EPub>
where
    EPub: EventPublisher,
{
    pub fn new(
        event_pub: Arc<EPub>,
        catalogue_repo: Arc<dyn CatalogueRepository>,
        collection_serv: Arc<dyn CollectionService>,
        publication_serv: Arc<dyn PublicationService>,
    ) -> Self {
        Container {
            event_pub,
            catalogue_repo,
            collection_serv,
            publication_serv,
        }
    }

    pub async fn subscribe<ES>(&self, event_sub: &ES) -> Result<()>
    where
        ES: EventSubscriber,
    {
        let handler =
            PublicationHandler::new(self.catalogue_repo.clone(), self.publication_serv.clone());
        event_sub.subscribe(Box::new(handler)).await?;

        let handler =
            CollectionHandler::new(self.catalogue_repo.clone(), self.collection_serv.clone());
        event_sub.subscribe(Box::new(handler)).await?;

        Ok(())
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn catalogue_repo(&self) -> &dyn CatalogueRepository {
        self.catalogue_repo.as_ref()
    }

    pub fn publication_serv(&self) -> &dyn PublicationService {
        self.publication_serv.as_ref()
    }
}
