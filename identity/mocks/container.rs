use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::Container;
use crate::infrastructure::persistence::inmem::{
    InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};
use crate::mocks::{FakePasswordHasher, FakeTokenEncoder};

pub fn container() -> Container<
    FakeEventPublisher,
    InMemRoleRepository,
    InMemTokenRepository,
    InMemUserRepository,
    FakePasswordHasher,
    FakeTokenEncoder,
> {
    Container::new(
        Arc::new(FakeEventPublisher::new()),
        InMemRoleRepository::new(),
        InMemTokenRepository::new(),
        InMemUserRepository::new(),
        FakePasswordHasher::new(),
        FakeTokenEncoder::new(),
    )
}
