use common::error::Error;
use common::model::{AggregateRoot, DefaultEvent};

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{Name, Page, PageNumber, Statistics, Synopsis, Tag};

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId, DefaultEvent>,
    name: Name,
    synopsis: Synopsis,
    author_id: AuthorId,
    statistics: Statistics,
    pages: Vec<Page>,
    category_id: CategoryId,
    tags: Vec<Tag>,
}

impl Publication {
    pub fn new(
        id: PublicationId,
        name: &str,
        synopsis: &str,
        author_id: AuthorId,
        _statistics: Statistics,
        category_id: CategoryId,
    ) -> Result<Publication, Error> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            name: Name::new(name)?,
            synopsis: Synopsis::new(synopsis)?,
            author_id,
            statistics: Statistics::new(),
            pages: Vec::new(),
            category_id,
            tags: Vec::new(),
        })
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn author(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), Error> {
        self.name = Name::new(name)?;
        Ok(())
    }

    pub fn set_synopsis(&mut self, synopsis: &str) -> Result<(), Error> {
        self.synopsis = Synopsis::new(synopsis)?;
        Ok(())
    }

    pub fn set_statistics(&mut self, statistics: Statistics) -> Result<(), Error> {
        self.statistics = statistics;
        Ok(())
    }

    pub fn add_page(&mut self, page: Page) -> Result<(), Error> {
        for p in self.pages.iter_mut() {
            if p.number() == page.number() {
                *p = page;
                return Ok(());
            }
        }

        self.pages.push(page);
        Ok(())
    }

    pub fn remove_page(&mut self, number: &PageNumber) -> Result<(), Error> {
        self.pages.retain(|page| page.number() != number);
        Ok(())
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) -> Result<(), Error> {
        self.tags = tags;
        Ok(())
    }
}
