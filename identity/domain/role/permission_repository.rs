use async_trait::async_trait;

use common::result::Result;

use crate::domain::role::Permission;

#[async_trait]
pub trait PermissionRepository: Sync + Send {
    async fn find_all(&self) -> Result<Vec<Permission>>;
}
