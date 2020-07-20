use crate::common::error::Error;

pub trait PasswordHasher {
    fn hash(&self, plain_password: &str) -> Result<String, Error>;
    fn compare(&self, hashed_password: &str, plain_password: &str) -> bool;
}
