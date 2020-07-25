use common::error::Error;
use common::model::AggregateRoot;

use crate::domain::author::{Author, AuthorID};
use crate::domain::category::{Category, CategoryID};
use crate::domain::publication::{Name, Page, PageNumber, Statistics, Synopsis, Tag};

pub type PublicationID = String;

pub struct Publication {
    base: AggregateRoot<PublicationID>,
    name: Name,
    synopsis: Synopsis,
    author_id: AuthorID,
    statistics: Statistics,
    pages: Vec<Page>,
    category_id: CategoryID,
    tags: Vec<Tag>,
}

impl Publication {
    pub fn new(
        id: PublicationID,
        name: &str,
        synopsis: &str,
        author_id: AuthorID,
        statistics: Statistics,
        category_id: CategoryID,
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

    pub fn author(&self) -> &AuthorID {
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
