use async_trait::async_trait;

use common::error::Error;

use crate::domain::role::{Role, RoleId};

#[async_trait]
pub trait RoleRepository {
    fn get_by_code(&self, code: &RoleId) -> Result<Role, Error>;
}
