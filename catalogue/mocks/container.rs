use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::InMemCatalogueRepository;

pub fn container() -> Container<FakeEventPublisher> {
    Container::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemCatalogueRepository::new()),
    )
}
