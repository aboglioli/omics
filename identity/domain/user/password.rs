use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Self> {
        if password.len() < 50 {
            return Err(Error::new("password", "not_hashed"));
        }

        Ok(Password {
            password: password.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid() {
        assert!(Password::new("plain-password").is_err());
    }
}
