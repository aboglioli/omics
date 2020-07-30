use async_trait::async_trait;

use common::error::Error;

use crate::domain::role::{Role, RoleId, RoleRepository};

pub struct InMemRoleRepository;

impl InMemRoleRepository {
    pub fn new() -> InMemRoleRepository {
        InMemRoleRepository
    }
}

#[async_trait]
impl RoleRepository for InMemRoleRepository {
    async fn get_by_code(&self, code: &RoleId) -> Result<Role, Error> {
        if code == "user" {
            return Ok(Role::new(code.clone(), "User")?);
        }
        Err(Error::internal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() {
        let repo = InMemRoleRepository::new();
        repo.get_by_code(&RoleId::from("user")).await.unwrap();
        assert!(repo.get_by_code(&RoleId::from("another")).await.is_err());
    }
}
