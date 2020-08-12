use std::hash::{Hash, Hasher};

use crate::error::Error;
use crate::result::Result;

#[derive(Debug, Clone, Eq)]
pub struct StringId {
    id: String,
}

impl StringId {
    pub fn new(id: &str) -> Result<Self> {
        if id.is_empty() {
            return Err(Error::new("id", "empty"));
        }

        if id.len() < 4 {
            return Err(Error::new("id", "too_short"));
        }

        Ok(StringId { id: id.to_owned() })
    }

    pub fn value(&self) -> &str {
        &self.id
    }
}

impl PartialEq for StringId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for StringId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
