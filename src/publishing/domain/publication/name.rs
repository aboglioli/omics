use crate::common::error::Error;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name, Error> {
        if name.len() < 4 {
            return Err(Error::application().set_code("name_short").clone());
        }

        Ok(Name {
            name: String::from(name),
        })
    }
}
