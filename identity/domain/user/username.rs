use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Username {
    username: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Username> {
        if username.len() < 4 {
            return Err(Error::new("username", "too_short"));
        }

        if username.len() > 24 {
            return Err(Error::new("username", "too_long"));
        }

        Ok(Username {
            username: username.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.username
    }
}
