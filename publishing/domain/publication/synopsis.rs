use common::error::Error;

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
