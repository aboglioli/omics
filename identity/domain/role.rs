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
    default: bool,
}

impl Role {
    pub fn new(name: Name) -> Result<Self> {
        let id = slugify(name.value());
        let id = RoleId::new(id)?;

        Ok(Role {
            base: AggregateRoot::new(id),
            name,
            permissions: Vec::new(),
            default: false,
        })
    }

    pub fn build(
        base: AggregateRoot<RoleId>,
        name: Name,
        permissions: Vec<Permission>,
        default: bool,
    ) -> Self {
        Role {
            base,
            name,
            permissions,
            default,
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
            if p.id() == permission_id || p.id() == "*" {
                return true;
            }
        }

        false
    }

    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn set_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        self.base.update();
        Ok(())
    }

    pub fn set_permissions(&mut self, permissions: Vec<Permission>) -> Result<()> {
        self.permissions = permissions;
        self.base.update();
        Ok(())
    }

    pub fn set_default(&mut self, default: bool) -> Result<()> {
        self.default = default;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.base.delete();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_role() {
        let r = Role::new(Name::new("Admin").unwrap()).unwrap();
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));
        assert_eq!(r.name().value(), "Admin");
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));
    }

    #[test]
    fn permissions() {
        let permissions = vec![
            Permission::new("edit_all_users", "Edit all users").unwrap(),
            Permission::new("edit_own_user", "Edit own user").unwrap(),
        ];
        let mut r = Role::new(Name::new("Admin").unwrap()).unwrap();
        r.set_permissions(permissions).unwrap();

        assert!(r.can("edit_all_users"));
        assert!(!r.can("edit_all_publications"));
    }
}
