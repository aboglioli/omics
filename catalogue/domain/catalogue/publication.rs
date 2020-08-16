use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::catalogue::{Author, Category, Statistics};

#[derive(Debug, Clone)]
pub struct Publication {
    id: String,
    author: Author,
    name: String,
    synopsis: String,
    category: Category,
    tags: Vec<String>,
    cover: String,
    statistics: Statistics,
    premium: bool,
    pages: usize,
    published_at: DateTime<Utc>,
}

impl Publication {
    pub fn new<S: Into<String>>(
        id: S,
        author: Author,
        name: S,
        synopsis: S,
        category: Category,
        tags: Vec<S>,
        cover: S,
        statistics: Statistics,
        premium: bool,
        pages: usize,
    ) -> Result<Self> {
        Ok(Publication {
            id: id.into(),
            author,
            name: name.into(),
            synopsis: synopsis.into(),
            category,
            tags: tags.into_iter().map(|tag| tag.into()).collect(),
            cover: cover.into(),
            statistics,
            premium,
            pages,
            published_at: Utc::now(),
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

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn is_premium(&self) -> bool {
        self.premium
    }

    pub fn pages(&self) -> usize {
        self.pages
    }
}
