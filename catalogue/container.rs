use std::sync::Arc;

use common::event::EventPublisher;

use crate::domain::catalogue::CatalogueRepository;

pub struct Container<EPub> {
    event_pub: Arc<EPub>,

    catalogue_repo: Arc<dyn CatalogueRepository>,
}

impl<EPub: EventPublisher> Container<EPub> {
    pub fn new(event_pub: Arc<EPub>, catalogue_repo: Arc<dyn CatalogueRepository>) -> Self {
        Container {
            event_pub,
            catalogue_repo,
        }
    }

    pub fn event_pub(&self) -> &EPub {
        &self.event_pub
    }

    pub fn catalogue_repo(&self) -> &dyn CatalogueRepository {
        self.catalogue_repo.as_ref()
    }
}
