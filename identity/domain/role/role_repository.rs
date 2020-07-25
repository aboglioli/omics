use crate::domain::role::{Role, RoleId};
use common::error::Error;

pub trait RoleRepository {
    fn get_by_code(&self, code: &RoleId) -> Result<Role, Error>;
}
