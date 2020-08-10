use common::error::Error;
use common::result::Result;

use crate::domain::user::{User, UserId};

pub fn is_authorized(auth_user: &User, user_id: &UserId) -> Result<()> {
    let guard =
        &auth_user.base().id() == user_id || auth_user.role().base().id().value() == "admin";

    if !guard {
        return Err(Error::new("user", "unauthorized"));
    }

    Ok(())
}
