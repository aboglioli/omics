mod permission;
mod repository;
pub use permission::*;
pub use repository::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type RoleId = StringId;

#[derive(Debug, Clone)]
pub struct Role {
    base: AggregateRoot<RoleId, Event>,
    name: String,
    permissions: Vec<Permission>,
}

impl Role {
    pub fn new<S: Into<String>>(code: RoleId, name: S) -> Result<Self> {
        let name = name.into();

        Ok(Role {
            base: AggregateRoot::new(code),
            name,
            permissions: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<RoleId, Event> {
        &self.base
    }

    pub fn is(&self, role_id: &str) -> bool {
        self.base().id().value() == role_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_permissions(&self, module: &str, permissions: &str) -> bool {
        for p in self.permissions.iter() {
            if p.module() == module && p.contains(permissions) {
                return true;
            }
        }
        false
    }

    pub fn add_permissions(&mut self, permission: Permission) {
        self.permissions.push(permission);
    }
}

#[cfg(test)]
mod tests {

    use common::result::Result;

    use super::*;

    #[test]
    fn create_role() -> Result<()> {
        let r = Role::new(RoleId::new("admin").unwrap(), "Administrator")?;
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));
        assert_eq!(r.name(), "Administrator");
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::new("admin").unwrap()));

        Ok(())
    }

    #[test]
    fn permissions() -> Result<()> {
        let pmod1 = Permission::new("mod1", "CRUD")?;
        let pmod2 = Permission::new("mod2", "CRD")?;
        let pmod3 = Permission::new("mod3", "R")?;
        let mut r = Role::new(RoleId::new("user").unwrap(), "User")?;
        r.add_permissions(pmod1);
        r.add_permissions(pmod2);
        r.add_permissions(pmod3);
        assert!(r.has_permissions("mod1", "cD"));
        assert!(r.has_permissions("mod1", "crud"));
        assert!(r.has_permissions("mod2", "Cd"));
        assert!(!r.has_permissions("mod2", "CdU"));
        assert!(!r.has_permissions("mod3", "C"));
        assert!(r.has_permissions("mod3", "r"));

        let pmod4 = Permission::new("mod3", "c")?;
        r.add_permissions(pmod4);
        assert!(r.has_permissions("mod3", "C"));

        Ok(())
    }
}
