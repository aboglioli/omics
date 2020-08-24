use serde::Serialize;

use common::result::Result;

use crate::application::dtos::{CategoryDto, PublicationDto};
use crate::domain::author::AuthorId;
use crate::domain::category::CategoryRepository;
use crate::domain::publication::PublicationRepository;

#[derive(Serialize)]
pub struct GetPublicationsResponse {
    publications: Vec<PublicationDto>,
}

pub struct GetPublications<'a> {
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetPublications<'a> {
    pub fn new(
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetPublications {
            category_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, author_id: String) -> Result<GetPublicationsResponse> {
        let author_id = AuthorId::new(author_id)?;
        let publications = self.publication_repo.find_by_author_id(&author_id).await?;

        let mut publication_dtos = Vec::new();
        for publication in publications.iter() {
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;
            publication_dtos
                .push(PublicationDto::from(publication).category(CategoryDto::from(&category)));
        }

        Ok(GetPublicationsResponse {
            publications: publication_dtos,
        })
    }
}
