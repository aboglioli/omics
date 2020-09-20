use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use shared::infrastructure::persistence::inmem::InMemUserRepository;
use shared::mocks::FakeUserService;

use crate::container::PublishingContainer;
use crate::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemInteractionRepository, InMemPublicationRepository, InMemReaderRepository,
};

pub fn container() -> PublishingContainer<FakeEventPublisher> {
    PublishingContainer::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemAuthorRepository::new()),
        Arc::new(InMemCategoryRepository::new()),
        Arc::new(InMemCollectionRepository::new()),
        Arc::new(InMemInteractionRepository::new()),
        Arc::new(InMemPublicationRepository::new()),
        Arc::new(InMemReaderRepository::new()),
        Arc::new(InMemUserRepository::new()),
        Arc::new(FakeUserService::new()),
    )
}
