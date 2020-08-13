use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemContentManagerRepository, InMemInteractionRepository, InMemPublicationRepository,
    InMemReaderRepository, InMemStatisticsRepository,
};

pub fn container() -> Container<
    FakeEventPublisher,
    InMemAuthorRepository,
    InMemCategoryRepository,
    InMemCollectionRepository,
    InMemContentManagerRepository,
    InMemInteractionRepository,
    InMemPublicationRepository,
    InMemReaderRepository,
    InMemStatisticsRepository,
> {
    Container::new(
        Arc::new(FakeEventPublisher::new()),
        InMemAuthorRepository::new(),
        InMemCategoryRepository::new(),
        InMemCollectionRepository::new(),
        InMemContentManagerRepository::new(),
        InMemInteractionRepository::new(),
        InMemPublicationRepository::new(),
        InMemReaderRepository::new(),
        InMemStatisticsRepository::new(),
    )
}
