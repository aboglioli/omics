use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemAdminRepository, InMemAuthorRepository, InMemCategoryRepository,
    InMemCollectionRepository, InMemContentManagerRepository, InMemInteractionRepository,
    InMemPublicationRepository, InMemReaderRepository,
};

#[allow(dead_code)]
pub fn container() -> Container<FakeEventPublisher> {
    Container::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemAdminRepository::new()),
        Arc::new(InMemAuthorRepository::new()),
        Arc::new(InMemCategoryRepository::new()),
        Arc::new(InMemCollectionRepository::new()),
        Arc::new(InMemContentManagerRepository::new()),
        Arc::new(InMemInteractionRepository::new()),
        Arc::new(InMemPublicationRepository::new()),
        Arc::new(InMemReaderRepository::new()),
    )
}
