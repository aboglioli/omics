use async_trait::async_trait;

use common::result::Result;

use crate::domain::role::{Role, RoleId};

#[async_trait]
pub trait RoleRepository {
    async fn get_by_code(&self, code: &RoleId) -> Result<Role>; // TODO: rename
}
