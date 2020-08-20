use regex::Regex;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Username {
    username: String,
}

impl Username {
    pub fn new<S: Into<String>>(username: S) -> Result<Self> {
        let username = username.into();

        if username.len() < 4 {
            return Err(Error::new("username", "too_short"));
        }

        if username.len() > 24 {
            return Err(Error::new("username", "too_long"));
        }

        match Regex::new("^[a-zA-Z0-9]+[a-zA-Z0-9-_.]*[a-zA-Z0-9]+$") {
            Ok(re) => {
                if !re.is_match(&username) {
                    return Err(Error::new("username", "invalid_characters"));
                }
            }
            Err(_e) => {
                return Err(Error::new("username", "invalid_regex"));
            }
        }

        Ok(Username { username })
    }

    pub fn value(&self) -> &str {
        &self.username
    }
}

impl ToString for Username {
    fn to_string(&self) -> String {
        self.value().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let username = Username::new("user-name.123_1984").unwrap();
        assert_eq!(username.value(), "user-name.123_1984");
    }

    #[test]
    fn valid() {
        assert!(Username::new("username").is_ok());
        assert!(Username::new("user123").is_ok());
        assert!(Username::new("11user").is_ok());
        assert!(Username::new("username.123").is_ok());
        assert!(Username::new("user-123").is_ok());
        assert!(Username::new("user_123").is_ok());
        assert!(Username::new("USER.789").is_ok());
        assert!(Username::new("USER.name").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(Username::new("úser").is_err());
        assert!(Username::new("us€r").is_err());
        assert!(Username::new("@user").is_err());
        assert!(Username::new("us/er").is_err());
        assert!(Username::new("-user").is_err());
        assert!(Username::new("_user").is_err());
        assert!(Username::new("user-").is_err());
        assert!(Username::new("user_").is_err());
        assert!(Username::new("user.").is_err());
        assert!(Username::new(".user").is_err());
        assert!(Username::new("-_.user").is_err());
        assert!(Username::new("user-_.").is_err());
        assert!(Username::new("-_.").is_err());
    }
}
