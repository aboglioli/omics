use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Biography {
    biography: String,
}

impl Biography {
    pub fn new<S: Into<String>>(biography: S) -> Result<Self> {
        let biography = biography.into();

        if biography.is_empty() {
            return Err(Error::new("biography", "empty"));
        }

        Ok(Biography { biography })
    }

    pub fn value(&self) -> &str {
        &self.biography
    }
}

impl ToString for Biography {
    fn to_string(&self) -> String {
        self.value().to_owned()
    }
}
