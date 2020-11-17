mod name;
mod permission;
mod permission_repository;
mod repository;
pub use name::*;
pub use permission::*;
pub use permission_repository::*;
pub use repository::*;

use slug::slugify;

use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type RoleId = StringId;

#[derive(Debug, Clone)]
pub struct Role {
    base: AggregateRoot<RoleId>,
    name: Name,
    permissions: Vec<Permission>,
}

impl Role {
    pub fn new(name: Name) -> Result<Self> {
        let id = slugify(name.value());
        let id = RoleId::new(id)?;

        Ok(Role {
            base: AggregateRoot::new(id),
            name,
            permissions: Vec::new(),
        })
    }

    pub fn build(base: AggregateRoot<RoleId>, name: Name, permissions: Vec<Permission>) -> Self {
        Role {
            base,
            name,
            permissions,
        }
    }

    pub fn base(&self) -> &AggregateRoot<RoleId> {
        &self.base
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn can<S: Into<String>>(&self, permission_id: S) -> bool {
        let permission_id = permission_id.into();

        for p in self.permissions.iter() {
            if p.id() == permission_id {
                return true;
            }
        }

        false
    }

    pub fn set_permissions(&mut self, permissions: Vec<Permission>) -> Result<()> {
        self.permissions = permissions;
        self.base.update();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_role() {
        let r = Role::new(RoleId::new("admin").unwrap(), Name::new("Administrator").unwrap()).unwrap();
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));
        assert_eq!(r.name().value(), "Administrator");
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));
    }

    #[test]
    fn permissions() {
        let permissions = vec![
            Permission::new("edit_all_users", "Edit all users").unwrap(),
            Permission::new("edit_own_user", "Edit own user").unwrap(),
        ];
        let mut r = Role::new(RoleId::new("admin").unwrap(), Name::new("Administrator").unwrap()).unwrap();
        r.set_permissions(permissions).unwrap();

        assert!(r.can("edit_all_users"));
        assert!(!r.can("edit_all_publications"));
    }
}
