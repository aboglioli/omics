use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::error::Error;
use crate::result::Result;

#[derive(Debug, Clone, Eq)]
pub struct StringId {
    id: String,
}

impl StringId {
    pub fn new<S: Into<String>>(id: S) -> Result<Self> {
        let id = id.into();

        if id.is_empty() {
            return Err(Error::new("id", "empty"));
        }

        if id.len() < 4 {
            return Err(Error::new("id", "too_short"));
        }

        Ok(StringId { id })
    }

    pub fn value(&self) -> &str {
        &self.id
    }

    pub fn to_uuid(&self) -> Result<Uuid> {
        Uuid::parse_str(self.value()).map_err(|err| Error::bad_format("id").wrap_raw(err))
    }
}

impl ToString for StringId {
    fn to_string(&self) -> String {
        self.id.to_owned()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crete() {
        let id = StringId::new("#id01").unwrap();
        assert_eq!(id.value(), "#id01");

        assert!(StringId::new(String::from("#id01")).is_ok());
        assert!(StringId::new("#id").is_err());
    }
}
