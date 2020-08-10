use async_trait::async_trait;

use common::result::Result;

use crate::domain::role::{Role, RoleId};

#[async_trait]
pub trait RoleRepository {
    async fn find_by_id(&self, id: &RoleId) -> Result<Role>;
}
