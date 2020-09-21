use serde::{Deserialize, Serialize};

use common::request::Include;
use common::result::Result;
use shared::domain::user::{UserId, UserRepository};

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::publication::{PublicationRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub author_id: Option<String>,
    pub category_id: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    publications: Vec<PublicationDto>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    publication_repo: &'a dyn PublicationRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        publication_repo: &'a dyn PublicationRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        Search {
            author_repo,
            category_repo,
            publication_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: Option<String>,
        cmd: SearchCommand,
        include: Include,
    ) -> Result<SearchResponse> {
        let is_content_manager = if let Some(auth_id) = &auth_id {
            let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            user.is_content_manager()
        } else {
            false
        };

        let mut publications = self
            .publication_repo
            .search(
                cmd.author_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.category_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.status.as_ref(),
                cmd.name.as_ref(),
            )
            .await?;

        publications = publications
            .into_iter()
            .filter(|publication| {
                if is_content_manager {
                    return true;
                }

                if let Some(auth_id) = &auth_id {
                    if publication.author_id().value() == auth_id {
                        return true;
                    }
                }

                matches!(publication.status_history().current(), Status::Published { .. })
            })
            .collect();

        let mut publication_dtos = Vec::new();

        for publication in publications.iter() {
            let mut publication_dto = PublicationDto::from(publication);

            if include.has("author") {
                let author = self.author_repo.find_by_id(publication.author_id()).await?;
                publication_dto = publication_dto.author(AuthorDto::from(&author));
            }

            if include.has("category") {
                let category = self
                    .category_repo
                    .find_by_id(publication.header().category_id())
                    .await?;
                publication_dto = publication_dto.category(CategoryDto::from(&category));
            }

            if let Some(auth_id) = &auth_id {
                if publication.author_id().value() == auth_id {
                    publication_dto = publication_dto.pages(&publication)
                }
            }

            publication_dtos.push(publication_dto);
        }

        Ok(SearchResponse {
            publications: publication_dtos,
        })
    }
}
