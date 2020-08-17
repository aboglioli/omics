use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Synopsis {
    synopsis: String,
}

impl Synopsis {
    pub fn new<S: Into<String>>(synopsis: S) -> Result<Self> {
        let synopsis = synopsis.into();

        if synopsis.len() < 4 {
            return Err(Error::new("synopsis", "too_short"));
        }

        Ok(Synopsis { synopsis })
    }

    pub fn value(&self) -> &str {
        &self.synopsis
    }
}
