use serde::Serialize;

use crate::domain::role::Role;
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub validated: bool,
    pub role: String,
}

impl UserDto {
    pub fn new(user: &User, include_email: bool) -> Self {
        UserDto {
            id: user.base().id().to_string(),
            username: user.identity().username().to_string(),
            email: if include_email {
                Some(user.identity().email().to_string())
            } else {
                None
            },
            name: user.person().map(|p| p.fullname().name().to_string()),
            lastname: user.person().map(|p| p.fullname().lastname().to_string()),
            validated: user.is_validated(),
            role: user.role().base().id().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
}

impl RoleDto {
    pub fn new(role: &Role) -> Self {
        RoleDto {
            id: role.base().id().to_string(),
            name: role.name().to_string(),
        }
    }
}
