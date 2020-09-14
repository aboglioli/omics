use bcrypt::{hash, verify, DEFAULT_COST};

use common::error::Error;
use common::result::Result;

use crate::domain::user::PasswordHasher;

#[derive(Default)]
pub struct BcryptHasher {
    cost: u32,
}

impl BcryptHasher {
    pub fn new() -> Self {
        BcryptHasher { cost: 4 }
    }
}

impl PasswordHasher for BcryptHasher {
    fn hash(&self, plain_password: &str) -> Result<String> {
        match hash(plain_password, self.cost) {
            Ok(hashed) => Ok(hashed),
            Err(err) => Err(Error::new("password", "hash").wrap_raw(err)),
        }
    }

    fn compare(&self, hashed_password: &str, plain_password: &str) -> bool {
        match verify(plain_password, hashed_password) {
            Ok(equal) => equal,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_compare() {
        let hasher = BcryptHasher {
            cost: 4, // minimum
        };

        let hashed = hasher.hash("P@asswd!").unwrap();
        assert!(hashed.len() > 20);
        assert!(hasher.compare(&hashed, "P@asswd!"));
    }
}
