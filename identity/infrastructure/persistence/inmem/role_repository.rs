use async_trait::async_trait;

use common::cache::{inmem::InMemCache, Cache};
use common::error::Error;
use common::result::Result;

use crate::domain::role::{Role, RoleId, RoleRepository};
use crate::mocks;

pub struct InMemRoleRepository {
    cache: InMemCache<RoleId, Role>,
}

impl InMemRoleRepository {
    pub fn new() -> Self {
        InMemRoleRepository {
            cache: InMemCache::new(),
        }
    }

    pub async fn populated() -> Self {
        let repo = Self::new();

        repo.save(&mut mocks::user_role()).await.unwrap();
        repo.save(&mut mocks::admin_role()).await.unwrap();

        repo
    }
}

#[async_trait]
impl RoleRepository for InMemRoleRepository {
    async fn find_by_id(&self, id: &RoleId) -> Result<Role> {
        self.cache
            .get(id)
            .await
            .ok_or(Error::new("role", "not_found"))
    }

    async fn save(&self, role: &mut Role) -> Result<()> {
        self.cache.set(role.base().id(), role.clone()).await
    }
}
