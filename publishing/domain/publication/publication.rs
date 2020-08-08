use common::event::Event;
use common::model::{AggregateRoot, StatusHistory};
use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::publication::{Name, Page, PublicationStatus, Statistics, Synopsis, Tag};

pub type PublicationId = String;

pub struct Publication {
    base: AggregateRoot<PublicationId, Event>,
    name: Name,
    synopsis: Synopsis,
    author_id: AuthorId,
    statistics: Statistics,
    pages: Vec<Page>,
    category_id: CategoryId,
    tags: Vec<Tag>,
    status: StatusHistory<PublicationStatus, String>,
    contract: bool,
}

impl Publication {
    pub fn new(
        id: PublicationId,
        name: Name,
        synopsis: Synopsis,
        author_id: AuthorId,
        category_id: CategoryId,
    ) -> Result<Publication> {
        Ok(Publication {
            base: AggregateRoot::new(id),
            name,
            synopsis,
            author_id,
            statistics: Statistics::default(),
            pages: Vec::new(),
            category_id,
            tags: Vec::new(),
            status: StatusHistory::new(PublicationStatus::Draft),
            contract: false,
        })
    }

    pub fn base(&self) -> &AggregateRoot<PublicationId, Event> {
        &self.base
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn synopsis(&self) -> &Synopsis {
        &self.synopsis
    }

    pub fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn category_id(&self) -> &CategoryId {
        &self.category_id
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn status(&self) -> &StatusHistory<PublicationStatus, String> {
        &self.status
    }

    pub fn has_contract(&self) -> bool {
        self.contract
    }

    pub fn set_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        Ok(())
    }

    pub fn set_synopsis(&mut self, synopsis: Synopsis) -> Result<()> {
        self.synopsis = synopsis;
        Ok(())
    }

    pub fn set_statistics(&mut self, statistics: Statistics) -> Result<()> {
        self.statistics = statistics;
        Ok(())
    }

    pub fn set_pages(&mut self, pages: Vec<Page>) -> Result<()> {
        self.pages = pages;
        Ok(())
    }

    pub fn set_cateogry(&mut self, category_id: CategoryId) -> Result<()> {
        self.category_id = category_id;
        Ok(())
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) -> Result<()> {
        self.tags = tags;
        Ok(())
    }

    pub fn add_contract(&mut self) -> Result<()> {
        self.contract = true;
        Ok(())
    }

    pub fn remove_contract(&mut self) -> Result<()> {
        self.contract = false;
        Ok(())
    }
}
