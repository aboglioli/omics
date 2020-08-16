use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::publication::PublicationRepository;

pub struct GetById<'a, ARepo, CRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CRepo, PRepo> GetById<'a, ARepo, CRepo, PRepo>
where
    ARepo: AuthorRepository,
    CRepo: CategoryRepository,
    PRepo: PublicationRepository,
{
    pub fn new(
        author_repo: &'a ARepo,
        category_repo: &'a CRepo,
        publication_repo: &'a PRepo,
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
        for publication in publications {
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;
            publication_dtos.push(PublicationDto::new(
                &publication,
                AuthorDto::new(&author, None),
                CategoryDto::new(&category),
                false,
                false,
            ));
        }

        Ok(AuthorDto::new(&author, Some(publication_dtos)))
    }
}
