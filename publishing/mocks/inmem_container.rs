use std::sync::Arc;

use common::mocks::FakeEventPublisher;
use common::result::Result;
use shared::infrastructure::persistence::inmem::InMemUserRepository;
use shared::mocks::FakeUserService;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemAuthorRepository, InMemCategoryRepository, InMemCollectionRepository,
    InMemInteractionRepository, InMemPublicationRepository, InMemReaderRepository,
};
use crate::mocks;

pub async fn inmem_container() -> Result<Container<FakeEventPublisher>> {
    let container = Container::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemAuthorRepository::new()),
        Arc::new(InMemCategoryRepository::new()),
        Arc::new(InMemCollectionRepository::new()),
        Arc::new(InMemInteractionRepository::new()),
        Arc::new(InMemPublicationRepository::new()),
        Arc::new(InMemReaderRepository::new()),
        Arc::new(InMemUserRepository::new()),
        Arc::new(FakeUserService::new()),
    );

    mocks::populate(&container).await?;

    Ok(container)
}
