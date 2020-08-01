use common::error::Error;
use common::result::Result;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name> {
        if name.len() < 4 {
            return Err(Error::application().set_code("name_short").build());
        }

        Ok(Name {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}
