use common::result::Result;

#[derive(Debug, Clone)]
pub struct Synopsis {
    synopsis: String,
}

impl Synopsis {
    pub fn new<S: Into<String>>(synopsis: S) -> Result<Self> {
        Ok(Synopsis {
            synopsis: synopsis.into(),
        })
    }

    pub fn value(&self) -> &str {
        &self.synopsis
    }
}
