use common::result::Result;

use crate::application::dtos::CatalogueDto;
use crate::domain::catalogue::CatalogueRepository;

pub struct Get<'a, CRepo> {
    catalogue_repo: &'a CRepo,
}

impl<'a, CRepo> Get<'a, CRepo>
where
    CRepo: CatalogueRepository,
{
    pub fn new(catalogue_repo: &'a CRepo) -> Self {
        Get { catalogue_repo }
    }

    pub async fn exec(&self) -> Result<CatalogueDto> {
        let catalogue = self.catalogue_repo.find().await?;
        Ok(CatalogueDto::new(&catalogue))
    }
}
