use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{Publication, PublicationId};

#[derive(Debug, Clone, Default)]
pub struct Preferences {
    category_ids: Vec<CategoryId>,
    publication_ids: Vec<PublicationId>,
}

impl Preferences {
    pub fn add_publication(&mut self, publication: &Publication) -> Result<()> {
        self.publication_ids.push(publication.base().id().clone());
        self.category_ids
            .push(publication.header().category_id().clone());
        Ok(())
    }

    pub fn category_ids(&self) -> &[CategoryId] {
        &self.category_ids
    }

    pub fn publication_ids(&self) -> &[PublicationId] {
        &self.publication_ids
    }
}
