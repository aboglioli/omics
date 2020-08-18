use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::publication::PublicationRepository;

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetById {
            author_repo,
            category_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, author_id: String) -> Result<AuthorDto> {
        let author_id = AuthorId::new(author_id)?;
        let author = self.author_repo.find_by_id(&author_id).await?;

        let publications = self.publication_repo.find_by_author_id(&author_id).await?;

        let mut publication_dtos = Vec::new();
        for publication in publications.iter() {
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;
            publication_dtos
                .push(PublicationDto::new(publication).category(CategoryDto::new(&category)));
        }

        Ok(AuthorDto::new(&author).publications(publication_dtos))
    }
}
