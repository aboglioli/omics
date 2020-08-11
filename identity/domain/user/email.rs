use regex::Regex;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Email {
    email: String,
}

impl Email {
    pub fn new(email: &str) -> Result<Self> {
        if email.len() < 5 {
            return Err(Error::new("email", "too_short"));
        }

        if email.len() > 64 {
            return Err(Error::new("email", "too_long"));
        }

        match Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$") {
            Ok(re) => {
                if !re.is_match(email) {
                    return Err(Error::new("email", "invalid"));
                }
            }
            Err(_) => {
                return Err(Error::new("email", "invalid_regex"));
            }
        }

        Ok(Email {
            email: email.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let email = Email::new("user@domain.com").unwrap();
        assert_eq!(email.value(), "user@domain.com");
    }

    #[test]
    fn valid() {
        assert!(Email::new("user@domain.com").is_ok());
        assert!(Email::new("u@domain.com").is_ok());
        assert!(Email::new("u@d.com").is_ok());
        assert!(Email::new("user.123@d.com").is_ok());
        assert!(Email::new("user_123.asd@domain-slashed.com").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(Email::new("user").is_err());
        assert!(Email::new("user@com").is_err());
        assert!(Email::new("user@.com").is_err());
        assert!(Email::new("user@domain").is_err());
        assert!(Email::new("user@d_s.com").is_err());
    }
}
