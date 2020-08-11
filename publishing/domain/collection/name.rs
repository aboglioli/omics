use common::result::Result;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Name {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}
