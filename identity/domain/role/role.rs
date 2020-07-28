use common::error::Error;
use common::event::BasicEvent;
use common::model::AggregateRoot;

use crate::domain::role::Permission;

pub type RoleId = String;

#[derive(Debug, Clone)]
pub struct Role {
    base: AggregateRoot<RoleId, BasicEvent>,
    name: String,
    permissions: Vec<Permission>,
}

impl Role {
    pub fn new(code: RoleId, name: &str) -> Result<Role, Error> {
        Ok(Role {
            base: AggregateRoot::new(code),
            name: String::from(name),
            permissions: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<RoleId, BasicEvent> {
        &self.base
    }

    pub fn name(&self) -> &String {
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
    use common::error::Error;

    use super::*;

    #[test]
    fn create_role() -> Result<(), Error> {
        let r = Role::new(RoleId::from("admin"), "Administrator")?;
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::from("admin")));
        assert_eq!(r.name(), "Administrator");
        assert_eq!(r.base(), &AggregateRoot::new(RoleId::from("admin")));

        Ok(())
    }

    #[test]
    fn permissions() -> Result<(), Error> {
        let pmod1 = Permission::new("mod1", "CRUD")?;
        let pmod2 = Permission::new("mod2", "CRD")?;
        let pmod3 = Permission::new("mod3", "R")?;
        let mut r = Role::new(RoleId::from("user"), "User")?;
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
