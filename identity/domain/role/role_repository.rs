use common::error::Error;
use crate::domain::role::{Role, RoleID};

pub trait RoleRepository {
    fn get_by_code(&self, code: RoleID) -> Result<Role, Error>;
}
