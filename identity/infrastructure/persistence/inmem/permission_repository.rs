use async_trait::async_trait;

use common::result::Result;

use crate::domain::role::{Permission, PermissionRepository};

pub struct InMemPermissionRepository {
    permissions: Vec<Permission>,
}

impl InMemPermissionRepository {
    pub fn new() -> Self {
        InMemPermissionRepository {
            permissions: Vec::new(),
        }
    }
}

impl Default for InMemPermissionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PermissionRepository for InMemPermissionRepository {
    async fn find_all(&self) -> Result<Vec<Permission>> {
        Ok(self.permissions.clone())
    }
}
