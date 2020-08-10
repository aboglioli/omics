use common::error::Error;
use common::result::Result;
use slug::slugify;

#[derive(Debug, Clone)]
pub struct Tag {
    slug: String,
    name: String,
}

impl Tag {
    pub fn new(name: &str) -> Result<Tag> {
        if name.is_empty() {
            return Err(Error::new("tag", "empty_name"));
        }

        let slug = slugify(name);
        Ok(Tag {
            slug,
            name: name.to_owned(),
        })
    }

    pub fn slug(&self) -> &String {
        &self.slug
    }

    pub fn name(&self) -> &String {
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
