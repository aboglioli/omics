use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();

        if name.len() > 24 {
            return Err(Error::new("username", "too_long"));
        }

        Ok(Name { name })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        self.value().to_owned()
    }
}
