use common::error::Error;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name, Error> {
        if name.len() < 4 {
            return Err(Error::application().set_code("name_short").build());
        }

        Ok(Name {
            name: name.to_owned(),
        })
    }
}

pub struct Synopsis {
    synopsis: String,
}

impl Synopsis {
    pub fn new(synopsis: &str) -> Result<Synopsis, Error> {
        if synopsis.len() < 4 {
            return Err(Error::application());
        }

        Ok(Synopsis {
            synopsis: synopsis.to_owned(),
        })
    }
}
