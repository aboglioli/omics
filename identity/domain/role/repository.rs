use async_trait::async_trait;

use common::result::Result;

use crate::domain::role::{Role, RoleId};

#[async_trait]
pub trait RoleRepository: Sync + Send {
    async fn find_all(&self) -> Result<Vec<Role>>;
    async fn find_by_id(&self, id: &RoleId) -> Result<Role>;

    async fn save(&self, role: &mut Role) -> Result<()>;
}
