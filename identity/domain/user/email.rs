use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Email {
    email: String,
}

impl Email {
    pub fn new(email: &str) -> Result<Email> {
        if email.len() < 4 {
            return Err(Error::new("email", "too_short"));
        }

        if email.len() > 64 {
            return Err(Error::new("email", "too_long"));
        }

        Ok(Email {
            email: email.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.email
    }
}
