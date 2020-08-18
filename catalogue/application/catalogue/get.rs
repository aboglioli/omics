use common::result::Result;

use crate::application::dtos::CatalogueDto;
use crate::domain::catalogue::CatalogueRepository;

pub struct Get<'a> {
    catalogue_repo: &'a dyn CatalogueRepository,
}

impl<'a> Get<'a> {
    pub fn new(catalogue_repo: &'a dyn CatalogueRepository) -> Self {
        Get { catalogue_repo }
    }

    pub async fn exec(&self) -> Result<CatalogueDto> {
        let catalogue = self.catalogue_repo.find().await?;
        Ok(CatalogueDto::new(&catalogue))
    }
}
