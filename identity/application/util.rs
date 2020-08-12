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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[test]
    fn owner() {
        let user = mocks::user1();
        assert!(is_authorized(&user, &user.base().id()).is_ok());
    }

    #[test]
    fn only_admin() {
        let admin = mocks::admin1();
        assert!(is_authorized(&admin, &UserId::new("#any-id").unwrap()).is_ok());
    }

    #[test]
    fn not_owner_or_admin() {
        let user = mocks::user1();
        assert!(is_authorized(&user, &UserId::new("#any-id").unwrap()).is_err());
    }
}
