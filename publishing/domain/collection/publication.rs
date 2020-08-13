use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::PublicationId;

#[derive(Debug, Clone)]
pub struct Publication {
    id: PublicationId,
    author_id: AuthorId,
    name: String,
    synopsis: String,
    category_id: CategoryId,
    tags: Vec<String>,
    cover: String,
    pages: u32,
}

impl Publication {
    pub fn new<S: Into<String>>(
        id: PublicationId,
        author_id: AuthorId,
        name: S,
        synopsis: S,
        category_id: CategoryId,
        tags: Vec<String>,
        cover: S,
        pages: u32,
    ) -> Result<Self> {
        Ok(Publication {
            id,
            author_id,
            name: name.into(),
            synopsis: synopsis.into(),
            category_id,
            tags,
            cover: cover.into(),
            pages,
        })
    }

    pub fn id(&self) -> &PublicationId {
        &self.id
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn synopsis(&self) -> &str {
        &self.synopsis
    }

    pub fn category_id(&self) -> &CategoryId {
        &self.category_id
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn cover(&self) -> &str {
        &self.cover
    }
}
