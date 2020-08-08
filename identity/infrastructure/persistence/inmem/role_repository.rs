use async_trait::async_trait;

use common::error::Error;
use common::result::Result;

use crate::domain::role::{Role, RoleId, RoleRepository};

pub struct InMemRoleRepository;

impl InMemRoleRepository {
    pub fn new() -> InMemRoleRepository {
        InMemRoleRepository
    }
}

#[async_trait]
impl RoleRepository for InMemRoleRepository {
    async fn find_by_id(&self, code: &RoleId) -> Result<Role> {
        if code == "user" {
            return Ok(Role::new(code.clone(), "User")?);
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
        repo.find_by_id(&RoleId::from("user")).await.unwrap();
        assert!(repo.find_by_id(&RoleId::from("another")).await.is_err());
    }
}
