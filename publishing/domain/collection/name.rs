use common::result::Result;

#[derive(Debug, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        Ok(Name { name: name.into() })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}
