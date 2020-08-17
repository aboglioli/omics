use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::content_manager::{ContentManagerId, ContentManagerRepository};
use crate::domain::publication::{PublicationRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    author_id: Option<String>,
    category_id: Option<String>,
    status: Option<String>,
    text: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    publications: Vec<PublicationDto>,
}

pub struct Search<'a, ARepo, CRepo, CMRepo, PRepo> {
    author_repo: &'a ARepo,
    category_repo: &'a CRepo,
    content_manager_repo: &'a CMRepo,
    publication_repo: &'a PRepo,
}

impl<'a, ARepo, CRepo, CMRepo, PRepo> Search<'a, ARepo, CRepo, CMRepo, PRepo>
where
    ARepo: AuthorRepository,
    CRepo: CategoryRepository,
    CMRepo: ContentManagerRepository,
    PRepo: PublicationRepository,
{
    pub fn new(
        author_repo: &'a ARepo,
        category_repo: &'a CRepo,
        content_manager_repo: &'a CMRepo,
        publication_repo: &'a PRepo,
    ) -> Self {
        Search {
            author_repo,
            category_repo,
            content_manager_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, viewer_id: String, cmd: SearchCommand) -> Result<SearchResponse> {
        let mut publications = Vec::new();
        let content_manager_id = ContentManagerId::new(&viewer_id)?;
        let is_content_manager = self
            .content_manager_repo
            .find_by_id(&content_manager_id)
            .await
            .is_ok();
        let author_id = AuthorId::new(&viewer_id)?;

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

        if let Some(status) = cmd.status {
            publications.extend(self.publication_repo.find_by_status(&status).await?);
        }

        if let Some(text) = cmd.text {
            publications.extend(self.publication_repo.search(&text).await?);
        }

        let publications = publications.into_iter().filter(|publication| {
            if is_content_manager || publication.author_id() == &author_id {
                return true;
            }

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
