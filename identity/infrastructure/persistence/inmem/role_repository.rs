use async_trait::async_trait;

use common::error::Error;
use common::result::Result;

use crate::domain::role::{Role, RoleId, RoleRepository};

#[derive(Default)]
pub struct InMemRoleRepository;

impl InMemRoleRepository {
    pub fn new() -> Self {
        InMemRoleRepository
    }
}

#[async_trait]
impl RoleRepository for InMemRoleRepository {
    async fn find_by_id(&self, id: &RoleId) -> Result<Role> {
        if id.value() == "user" {
            return Ok(Role::new(id.clone(), "User")?);
        }
        Err(Error::new("role", "not_found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() {
        let repo = InMemRoleRepository::new();
        repo.find_by_id(&RoleId::new("user").unwrap())
            .await
            .unwrap();
        assert!(repo
            .find_by_id(&RoleId::new("another").unwrap())
            .await
            .is_err());
    }
}
