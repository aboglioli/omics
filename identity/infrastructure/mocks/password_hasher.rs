use crate::domain::user::PasswordHasher;
use common::error::Error;

pub struct FakePasswordHasher;

impl FakePasswordHasher {
    pub fn new() -> FakePasswordHasher {
        FakePasswordHasher
    }
}

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, plain_pasword: &str) -> Result<String, Error> {
        Ok(format!("$${:X>50}##", plain_pasword))
    }

    fn compare(&self, hashed_password: &str, plain_pasword: &str) -> bool {
        hashed_password == format!("$${:X>50}##", plain_pasword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let ph = FakePasswordHasher::new();
        let hashed_password = ph.hash("abc123")?;
        assert!(hashed_password.contains("abc123"));
        assert!(hashed_password.len() > 50);

        assert!(ph.compare(&hashed_password, "abc123"));
        assert!(!ph.compare(&hashed_password, "xyz123"));

        Ok(())
    }
}
