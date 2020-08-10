use common::error::Error;
use common::result::Result;

pub struct Synopsis {
    synopsis: String,
}

impl Synopsis {
    pub fn new(synopsis: &str) -> Result<Synopsis> {
        if synopsis.len() < 4 {
            return Err(Error::new("synopsis", "too_short"));
        }

        Ok(Synopsis {
            synopsis: synopsis.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.synopsis
    }
}
