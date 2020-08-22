use serde::{Deserialize, Serialize};

use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::content_manager::{ContentManagerId, ContentManagerRepository};
use crate::domain::publication::{PublicationRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    author_id: Option<String>,
    category_id: Option<String>,
    status: Option<String>,
    name: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    publications: Vec<PublicationDto>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    content_manager_repo: &'a dyn ContentManagerRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        content_manager_repo: &'a dyn ContentManagerRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        Search {
            author_repo,
            category_repo,
            content_manager_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self, auth_id: String, cmd: SearchCommand) -> Result<SearchResponse> {
        let content_manager_id = ContentManagerId::new(&auth_id)?;
        let is_content_manager = self
            .content_manager_repo
            .find_by_id(&content_manager_id)
            .await
            .is_ok();
        let author_id = AuthorId::new(&auth_id)?;

        let mut publications = self.publication_repo.find_all().await?;

        if let Some(author_id) = cmd.author_id {
            publications = publications
                .into_iter()
                .filter(|publication| publication.author_id().value() == author_id)
                .collect();
        }

        if let Some(category_id) = cmd.category_id {
            publications = publications
                .into_iter()
                .filter(|publication| publication.header().category_id().value() == category_id)
                .collect();
        }

        if let Some(status) = cmd.status {
            publications = publications
                .into_iter()
                .filter(|publication| {
                    publication.status_history().current().status().to_string() == status
                })
                .collect();
        }

        if let Some(name) = cmd.name {
            publications = publications
                .into_iter()
                .filter(|publication| publication.header().name().value().contains(&name))
                .collect();
        }

        publications = publications
            .into_iter()
            .filter(|publication| {
                if is_content_manager || publication.author_id() == &author_id {
                    return true;
                }

                matches!(publication.status_history().current().status(), Status::Published { .. })
            })
            .collect();

        let mut publication_dtos = Vec::new();
        for publication in publications.iter() {
            let author = self.author_repo.find_by_id(publication.author_id()).await?;
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;

            publication_dtos.push(
                PublicationDto::from(publication)
                    .author(AuthorDto::from(&author))
                    .category(CategoryDto::from(&category)),
            );
        }

        Ok(SearchResponse {
            publications: publication_dtos,
        })
    }
}
