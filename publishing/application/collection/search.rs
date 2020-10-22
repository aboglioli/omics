use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{Include, PaginationParams, PaginationResponse};
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, CollectionDto};
use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::collection::{CollectionOrderBy, CollectionRepository};
use crate::domain::publication::{PublicationId, Tag};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub author_id: Option<String>,
    pub category_id: Option<String>,
    pub publication_id: Option<String>,
    pub tag: Option<String>,
    pub name: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        Search {
            author_repo,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        _auth_id: Option<String>,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<CollectionDto>> {
        let pagination_collections = self
            .collection_repo
            .search(
                cmd.author_id.map(AuthorId::new).transpose()?.as_ref(),
                cmd.category_id.map(CategoryId::new).transpose()?.as_ref(),
                cmd.publication_id
                    .map(PublicationId::new)
                    .transpose()?
                    .as_ref(),
                cmd.tag.map(Tag::new).transpose()?.as_ref(),
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
                    .map(|o| CollectionOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::new(
            pagination_collections.offset(),
            pagination_collections.limit(),
            pagination_collections.total(),
            pagination_collections.matching_criteria(),
        );

        for collection in pagination_collections.into_items().into_iter() {
            let mut collection_dto = CollectionDto::from(&collection);

            if include.has("author") {
                let author = self.author_repo.find_by_id(collection.author_id()).await?;
                collection_dto = collection_dto.author(AuthorDto::from(&author));
            }

            if include.has("category") {
                let category = self
                    .category_repo
                    .find_by_id(collection.header().category_id())
                    .await?;
                collection_dto = collection_dto.category(CategoryDto::from(&category));
            }

            res.add_item(collection_dto);
        }

        Ok(res)
    }
}
