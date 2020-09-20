use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};

use crate::mocks::{FakePasswordHasher, FakeTokenEncoder};

pub fn container() -> Container<FakeEventPublisher> {
    Container::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemRoleRepository::new()),
        Arc::new(InMemTokenRepository::new()),
        Arc::new(InMemUserRepository::new()),
        Arc::new(FakePasswordHasher::new()),
        Arc::new(FakeTokenEncoder::new()),
    )
}
