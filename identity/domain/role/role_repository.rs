use crate::domain::role::{Role, RoleID};
use common::error::Error;

pub trait RoleRepository {
    fn get_by_code(&self, code: &RoleID) -> Result<Role, Error>;
}
