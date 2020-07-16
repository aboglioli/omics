use crate::common::error::Error;
use crate::common::model::{Entity, ID};
use crate::identity::domain::role::Permission;

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
        return false;
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
