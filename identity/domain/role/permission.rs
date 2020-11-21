use serde::{Deserialize, Serialize};

use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    id: String,
    name: String,
}

impl Permission {
    pub fn new<S: Into<String>>(id: S, name: S) -> Result<Self> {
        Ok(Permission {
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
