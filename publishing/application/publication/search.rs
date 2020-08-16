use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::publication::{PublicationRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    author_id: Option<String>,
    category_id: Option<String>,
    text: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    publications: Vec<PublicationDto>,
}

pub struct Search<'a, ARepo, CRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CRepo, PRepo> Search<'a, ARepo, CRepo, PRepo>
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
        Search {
            author_repo,
            category_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, cmd: SearchCommand) -> Result<SearchResponse> {
        let mut publications = Vec::new();

        if let Some(author_id) = cmd.author_id {
            let author_id = AuthorId::new(author_id)?;
            publications.extend(self.publication_repo.find_by_author_id(&author_id).await?);
        }

        if let Some(category_id) = cmd.category_id {
            let category_id = CategoryId::new(category_id)?;
            publications.extend(
                self.publication_repo
                    .find_by_category_id(&category_id)
                    .await?,
            );
        }

        if let Some(text) = cmd.text {
            publications.extend(self.publication_repo.search(&text).await?);
        }

        let publications = publications.into_iter().filter(|publication| {
            matches!(publication.status_history().current().status(), Status::Published { .. })
        });

        let mut publication_dtos = Vec::new();
        for publication in publications {
            let author = self.author_repo.find_by_id(publication.author_id()).await?;
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

        Ok(SearchResponse {
            publications: publication_dtos,
        })
    }
}
