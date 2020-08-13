use common::error::Error;
use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{Image, Name, Synopsis, Tag};

#[derive(Debug, Clone)]
pub struct Header {
    name: Name,
    synopsis: Synopsis,
    category_id: CategoryId,
    tags: Vec<Tag>,
    cover: Image,
}

impl Header {
    pub fn new(
        name: Name,
        synopsis: Synopsis,
        category_id: CategoryId,
        tags: Vec<Tag>,
        cover: Image,
    ) -> Result<Header> {
        if tags.len() > 5 {
            return Err(Error::new("publication", "maximum_tags_exceeded"));
        }

        Ok(Header {
            name,
            synopsis,
            category_id,
            tags,
            cover,
        })
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn category_id(&self) -> &CategoryId {
        &self.category_id
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn cover(&self) -> &Image {
        &self.cover
    }
}
