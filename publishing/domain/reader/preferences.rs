use common::result::Result;

use crate::domain::category::CategoryId;
use crate::domain::publication::{Publication, PublicationId};

#[derive(Debug, Clone, Default)]
pub struct Preferences {
    categories: Vec<CategoryId>,
    publications: Vec<PublicationId>,
}

impl Preferences {
    pub fn add_publication(&mut self, publication: &Publication) -> Result<()> {
        self.publications.push(publication.base().id().clone());
        self.categories
            .push(publication.header().category_id().clone());
        Ok(())
    }
}
