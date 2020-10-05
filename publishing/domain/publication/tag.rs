use serde::{Deserialize, Serialize};
use slug::slugify;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    slug: String,
    name: String,
}

impl Tag {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(Error::new("tag", "empty_name"));
        }

        let slug = slugify(&name);

        Ok(Tag { slug, name })
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() -> Result<()> {
        let tag = Tag::new("New fancy tag")?;
        assert_eq!(tag.name(), "New fancy tag");
        assert_eq!(tag.slug(), "new-fancy-tag");

        assert!(Tag::new("").is_err());

        Ok(())
    }
}
