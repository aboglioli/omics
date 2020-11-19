use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{Include, PaginationParams, PaginationResponse};
use common::result::Result;
use identity::domain::user::UserRepository;
use identity::UserIdAndRole;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::CategoryRepository;
use crate::domain::publication::{PublicationOrderBy, PublicationRepository, Status, Tag};

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
        user_id_and_role: Option<UserIdAndRole>,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<PublicationDto>> {
        if let Some((_, auth_role)) = &user_id_and_role {
            if !auth_role.can("search_publications") {
                return Err(Error::unauthorized());
            }
        }

        let is_reader_author =
            if let (Some((auth_id, _)), Some(author_id)) = (&user_id_and_role, &cmd.author_id) {
                auth_id.value() == author_id
            } else {
                false
            };

        let is_content_manager = if let Some((_, auth_role)) = &user_id_and_role {
            auth_role.can("approve_publication")
        } else {
            false
        };

        let status = if is_content_manager || is_reader_author {
            cmd.status.map(|s| Status::from_str(&s)).transpose()?
        } else {
            Some(Status::Published {
                admin_id: None,
                comment: None,
            })
        };

        let pagination_publications = self
            .publication_repo
            .search(
                cmd.author_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.category_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.tag.map(Tag::new).transpose()?.as_ref(),
                status.as_ref(),
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
                pagination.offset(),
                pagination.limit(),
                pagination
                    .order_by()
                    .map(|o| PublicationOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::new(
            pagination_publications.offset(),
            pagination_publications.limit(),
            pagination_publications.total(),
            pagination_publications.matching_criteria(),
        );

        for publication in pagination_publications.into_items().into_iter() {
            let mut publication_dto = PublicationDto::from(&publication);

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

            if let Some((auth_id, _)) = &user_id_and_role {
                if publication.author_id() == auth_id {
                    publication_dto = publication_dto.pages(&publication)
                }
            }

            res.add_item(publication_dto);
        }

        Ok(res)
    }
}
