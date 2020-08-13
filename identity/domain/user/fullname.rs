use regex::Regex;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Fullname {
    name: String,
    lastname: String,
}

impl Fullname {
    pub fn new<S: Into<String>>(name: S, lastname: S) -> Result<Self> {
        let name = name.into();
        let lastname = lastname.into();

        let mut err = Error::new("fullname", "invalid");

        if name.len() < 2 {
            err.add_context("name", "too_short");
        }

        if name.len() > 64 {
            err.add_context("name", "too_long");
        }

        if lastname.len() < 2 {
            err.add_context("lastname", "too_short");
        }

        if lastname.len() > 64 {
            err.add_context("lastname", "too_long");
        }

        match Regex::new("^[a-zA-Z]+ *[a-zA-Z]+$") {
            Ok(re) => {
                if !re.is_match(&name) {
                    err.add_context("name", "invalid_characters");
                }
                if !re.is_match(&lastname) {
                    err.add_context("lastname", "invalid_characters");
                }
            }
            Err(_) => {
                err.add_context("name-lastname", "invalid_regex");
            }
        }

        if err.has_context() {
            return Err(err);
        }

        Ok(Fullname { name, lastname })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lastname(&self) -> &str {
        &self.lastname
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let fullname = Fullname::new("User", "One").unwrap();
        assert_eq!(fullname.name(), "User");
        assert_eq!(fullname.lastname(), "One");
    }

    #[test]
    fn valid() {
        assert!(Fullname::new(String::from("User"), String::from("One")).is_ok());
        assert!(Fullname::new("User", "One").is_ok());
        assert!(Fullname::new("User", "On").is_ok());
        assert!(Fullname::new("Us", "On").is_ok());
        assert!(Fullname::new("User Valid", "One").is_ok());
        assert!(Fullname::new("Lee", "Jo").is_ok());
        assert!(Fullname::new("Alan Daniel", "Boglioli Caffe").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(Fullname::new("U", "O").is_err());
        assert!(Fullname::new("U", "One").is_err());
        assert!(Fullname::new("User-Invalid", "One").is_err());
        assert!(Fullname::new("User", "One_Two").is_err());
        assert!(Fullname::new("User", "One@Two").is_err());
        assert!(Fullname::new(" User", "One ").is_err());
    }
}
