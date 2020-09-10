use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemInteractionRepository, InMemPublicationRepository, InMemReaderRepository,
    InMemUserRepository,
};
use crate::mocks::FakeUserService;

#[allow(dead_code)]
pub fn container() -> Container<FakeEventPublisher> {
    Container::new(
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
