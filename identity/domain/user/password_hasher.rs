use common::result::Result;

pub trait PasswordHasher: Sync + Send {
    fn hash(&self, plain_password: &str) -> Result<String>;
    fn compare(&self, hashed_password: &str, plain_password: &str) -> bool;
}
