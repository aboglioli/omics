use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new<S: Into<String>>(password: S) -> Result<Self> {
        let password = password.into();

        if password.len() < 50 {
            return Err(Error::new("password", "not_hashed"));
        }

        Ok(Password { password })
    }

    pub fn value(&self) -> &str {
        &self.password
    }
}

impl ToString for Password {
    fn to_string(&self) -> String {
        self.value().to_owned()
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
