use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{PaginationParams, PaginationResponse};
use common::result::Result;
use identity::UserIdAndRole;

use crate::application::dtos::AuthorDto;
use crate::domain::author::{AuthorOrderBy, AuthorRepository};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub name: Option<String>,
    pub publications_gt: Option<u32>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
}
impl<'a> Search<'a> {
    pub fn new(author_repo: &'a dyn AuthorRepository) -> Self {
        Search { author_repo }
    }

    pub async fn exec(
        &self,
        _user_id_and_role: Option<UserIdAndRole>,
        cmd: SearchCommand,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<AuthorDto>> {
        let pagination_authors = self
            .author_repo
            .search(
                cmd.name.as_ref(),
                cmd.publications_gt,
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
                    .map(|o| AuthorOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::new(
            pagination_authors.offset(),
            pagination_authors.limit(),
            pagination_authors.total(),
            pagination_authors.matching_criteria(),
        );

        for author in pagination_authors.into_items().into_iter() {
            // TODO: change this
            if author.username().starts_with("admin")
                || author.username().starts_with("content-manager")
            {
                continue;
            }

            res.add_item(AuthorDto::from(&author));
        }

        Ok(res)
    }
}
