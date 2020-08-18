use async_trait::async_trait;

use common::error::Error;
use common::event::{Event, EventHandler};
use common::result::Result;
use shared::event::PublicationEvent;

use crate::domain::catalogue::{CatalogueRepository, PublicationService};

pub struct PublicationHandler<'a, CRepo, PServ> {
    catalogue_repo: &'a CRepo,

    publication_serv: &'a PServ,
}

impl<'a, CRepo, PServ> PublicationHandler<'a, CRepo, PServ> {
    pub fn new(catalogue_repo: &'a CRepo, publication_serv: &'a PServ) -> Self {
        PublicationHandler {
            catalogue_repo,
            publication_serv,
        }
    }
}

#[async_trait]
impl<'a, CRepo, PServ> EventHandler for PublicationHandler<'a, CRepo, PServ>
where
    CRepo: CatalogueRepository + Sync + Send,
    PServ: PublicationService + Sync + Send,
{
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
