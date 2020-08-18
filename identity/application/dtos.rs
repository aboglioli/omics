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
            id: user.base().id().value().to_owned(),
            username: user.identity().username().value().to_owned(),
            email: if include_email {
                Some(user.identity().email().value().to_owned())
            } else {
                None
            },
            name: user.person().map(|p| p.fullname().name().to_owned()),
            lastname: user.person().map(|p| p.fullname().lastname().to_owned()),
            validated: user.is_validated(),
            role: user.role().base().id().value().to_owned(),
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
            id: role.base().id().value().to_owned(),
            name: role.name().to_owned(),
        }
    }
}
