use crate::domain::role::{Role, RoleID, RoleRepository};
use common::error::Error;

pub struct InMemRoleRepository;

impl InMemRoleRepository {
    pub fn new() -> InMemRoleRepository {
        InMemRoleRepository
    }
}

impl RoleRepository for InMemRoleRepository {
    fn get_by_code(&self, code: RoleID) -> Result<Role, Error> {
        if code == "user" {
            return Ok(Role::new(code, "User")?);
        }
        Err(Error::internal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() -> Result<(), Error> {
        let repo = InMemRoleRepository::new();
        repo.get_by_code(RoleID::from("user"))?;
        assert!(repo.get_by_code(RoleID::from("another")).is_err());

        Ok(())
    }
}
