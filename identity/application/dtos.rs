use serde::Serialize;

use crate::domain::role::{Permission, Role};
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub profile_image: Option<String>,
    pub validated: bool,
    pub role_id: Option<String>,
    pub role: Option<RoleDto>,
    pub payment_email: Option<String>,
    pub flag: Option<i64>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl UserDto {
    pub fn role(mut self, role: RoleDto) -> Self {
        self.role_id = None;
        self.role = Some(role);
        self
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        UserDto {
            id: user.base().id().to_string(),
            username: user.identity().username().to_string(),
            email: user.identity().email().to_string(),
            name: user.person().map(|p| p.fullname().name().to_string()),
            lastname: user.person().map(|p| p.fullname().lastname().to_string()),
            birthdate: user
                .person()
                .map(|p| p.birthdate().map(|b| b.to_string()))
                .flatten(),
            gender: user
                .person()
                .map(|p| p.gender().map(|g| g.to_string()))
                .flatten(),
            biography: user
                .person()
                .map(|p| p.biography().map(|b| b.to_string()))
                .flatten(),
            profile_image: user
                .person()
                .map(|p| p.profile_image().map(|i| i.to_string()))
                .flatten(),
            validated: user.is_validated(),
            role_id: Some(user.role_id().to_string()),
            role: None,
            payment_email: user.payment_email().map(|p| p.to_string()),
            flag: user.flag(),
            created_at: user.base().created_at().to_rfc3339(),
            updated_at: user.base().updated_at().map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Serialize)]
pub struct PermissionDto {
    pub id: String,
    pub name: String,
}

impl PermissionDto {
    pub fn from(permission: &Permission) -> PermissionDto {
        PermissionDto {
            id: permission.id().to_string(),
            name: permission.name().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
    pub permissions: Vec<PermissionDto>,
    pub default: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<&Role> for RoleDto {
    fn from(role: &Role) -> Self {
        RoleDto {
            id: role.base().id().to_string(),
            name: role.name().to_string(),
            permissions: role.permissions().iter().map(PermissionDto::from).collect(),
            default: role.is_default(),
            created_at: role.base().created_at().to_rfc3339(),
            updated_at: role.base().updated_at().map(|d| d.to_rfc3339()),
        }
    }
}
