use common::error::Error;
use slug::slugify;

#[derive(Debug, Clone)]
pub struct Tag {
    slug: String,
    name: String,
}

impl Tag {
    pub fn new(name: &str) -> Result<Tag, Error> {
        if name.is_empty() {
            return Err(Error::application().add_context("name", "empty").build());
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
    fn create() -> Result<(), Error> {
        let tag = Tag::new("New fancy tag")?;
        assert_eq!(tag.name(), "New fancy tag");
        assert_eq!(tag.slug(), "new-fancy-tag");

        assert!(Tag::new("").is_err());

        Ok(())
    }
}
