use std::sync::Arc;

use common::mocks::FakeEventPublisher;

use crate::container::IdentityContainer;
use crate::infrastructure::persistence::inmem::{
    InMemPermissionRepository, InMemRoleRepository, InMemTokenRepository, InMemUserRepository,
};

use crate::mocks::{FakePasswordHasher, FakeTokenEncoder};

pub fn container() -> IdentityContainer<FakeEventPublisher> {
    IdentityContainer::new(
        Arc::new(FakeEventPublisher::new()),
        Arc::new(InMemPermissionRepository::new()),
        Arc::new(InMemRoleRepository::new()),
        Arc::new(InMemTokenRepository::new()),
        Arc::new(InMemUserRepository::new()),
        Arc::new(FakePasswordHasher::new()),
        Arc::new(FakeTokenEncoder::new()),
    )
}
