use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();

        if name.len() < 2 {
            return Err(Error::new("name", "too_short"));
        }

        Ok(Name { name })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}
