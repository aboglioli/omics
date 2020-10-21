use std::str::FromStr;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::request::{Include, PaginationResponse, PaginationParams};
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::publication::{PublicationRepository, Status, Tag, PublicationOrderBy};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub author_id: Option<String>,
    pub category_id: Option<String>,
    pub tag: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
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
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<PublicationDto>> {
        // TODO: filter by status instead of doing it manually
        let is_content_manager = if let Some(auth_id) = &auth_id {
            let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            user.is_content_manager()
        } else {
            false
        };

        let mut pagination_publications = self
            .publication_repo
            .search(
                cmd.author_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.category_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.tag.map(Tag::new).transpose()?.as_ref(),
                cmd.status.as_ref(),
                cmd.name.as_ref(),
                cmd.date_from
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_from").wrap_raw(err))?
                    .as_ref(),
                cmd.date_to
                    .map(|d| DateTime::from_str(&d))
                    .transpose()
                    .map_err(|err| Error::bad_format("date_to").wrap_raw(err))?
                    .as_ref(),
                pagination.offset,
                pagination.limit,
                pagination
                    .order_by
                    .map(|o| PublicationOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let publications = pagination_publications
            .items()
            .iter()
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

        if let Some(order_by) = pagination.order_by {
            match order_by.as_ref() {
                "most_viewed" => {
                    publications
                        .sort_by(|a, b| b.statistics().views().cmp(&a.statistics().views()));
                }
                "most_liked" => {
                    publications
                        .sort_by(|a, b| b.statistics().likes().cmp(&a.statistics().likes()));
                }
                "newest" => {
                    publications.reverse();
                }
                "best_reviews" => {
                    publications.sort_by(|a, b| {
                        b.statistics()
                            .stars()
                            .partial_cmp(&a.statistics().stars())
                            .unwrap()
                    });
                }
                _ => {}
            }
        }

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
