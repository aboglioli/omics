use common::error::Error;
use common::model::{Entity, ID};
use identity::domain::role::Permission;

pub type RoleID = String;

pub struct Role {
    code: ID<RoleID>,
    name: String,
    permissions: Vec<Permission>,
}

impl Role {
    pub fn new(code: RoleID, name: &str) -> Result<Role, Error> {
        Ok(Role {
            code: ID::new(code),
            name: String::from(name),
            permissions: Vec::new(),
        })
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

impl Entity<RoleID> for Role {
    fn id(&self) -> &ID<RoleID> {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use crate::common::error::Error;
    use crate::common::model::{Entity, ID};

    use super::*;

    #[test]
    fn create_role() -> Result<(), Error> {
        let r = Role::new(RoleID::from("admin"), "Administrator")?;
        assert_eq!(r.id(), &ID::new(RoleID::from("admin")));
        assert_eq!(r.name(), "Administrator");
        assert!(r.eq_id(RoleID::from("admin")));

        Ok(())
    }

    #[test]
    fn permissions() -> Result<(), Error> {
        let pmod1 = Permission::new("mod1", "CRUD")?;
        let pmod2 = Permission::new("mod2", "CRD")?;
        let pmod3 = Permission::new("mod3", "R")?;
        let mut r = Role::new(RoleID::from("user"), "User")?;
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
