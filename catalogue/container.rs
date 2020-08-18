use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::catalogue::CatalogueRepository;

pub struct Container<EPub, CRepo> {
    event_pub: Arc<EPub>,

    catalogue_repo: Arc<CRepo>,
}

impl<EPub, CRepo> Container<EPub, CRepo>
where
    EPub: EventPublisher,
    CRepo: CatalogueRepository,
{
    pub fn new(event_pub: Arc<EPub>, catalogue_repo: Arc<CRepo>) -> Self {
        Container {
            event_pub,
            catalogue_repo,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn catalogue_repo(&self) -> &CRepo {
        &self.catalogue_repo
    }
}
