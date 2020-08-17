use common::result::Result;

use crate::domain::catalogue::{Author, Category};

#[derive(Debug, Clone)]
pub struct Collection {
    id: String,
    author: Author,
    name: String,
    synopsis: String,
    category: Category,
    tags: Vec<String>,
    cover: String,
    publications: usize,
}

impl Collection {
    pub fn new<S: Into<String>>(
        id: S,
        author: Author,
        name: S,
        synopsis: S,
        category: Category,
        tags: Vec<S>,
        cover: S,
        publications: usize,
    ) -> Result<Self> {
        Ok(Collection {
            id: id.into(),
            author,
            name: name.into(),
            synopsis: synopsis.into(),
            category,
            tags: tags.into_iter().map(|tag| tag.into()).collect(),
            cover: cover.into(),
            publications,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn author(&self) -> &Author {
        &self.author
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn synopsis(&self) -> &str {
        &self.synopsis
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn cover(&self) -> &str {
        &self.cover
    }

    pub fn publications(&self) -> usize {
        self.publications
    }
}
