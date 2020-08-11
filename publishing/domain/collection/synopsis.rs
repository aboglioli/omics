use common::result::Result;

pub struct Synopsis {
    synopsis: String,
}

impl Synopsis {
    pub fn new(synopsis: &str) -> Result<Self> {
        Ok(Synopsis {
            synopsis: synopsis.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.synopsis
    }
}
