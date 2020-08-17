use common::result::Result;

#[derive(Debug, Clone)]
pub struct Category {
    id: String,
    name: String,
}

impl Category {
    pub fn new<S: Into<String>>(id: S, name: S) -> Result<Self> {
        Ok(Category {
            id: id.into(),
            name: name.into(),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
