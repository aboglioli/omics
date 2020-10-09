use std::str::FromStr;

use chrono::DateTime;
use serde::{Deserialize, Serialize};

use common::error::Error;
use common::request::PaginationParams;
use common::result::Result;

use crate::application::dtos::AuthorDto;
use crate::domain::author::AuthorRepository;

#[derive(Deserialize)]
pub struct SearchCommand {
    pub name: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    authors: Vec<AuthorDto>,
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
        _auth_id: Option<String>,
        cmd: SearchCommand,
        pagination: PaginationParams,
    ) -> Result<SearchResponse> {
        let mut authors = self
            .author_repo
            .search(
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
            )
            .await?;

        if let Some(order_by) = pagination.order_by {
            match order_by.as_ref() {
                "followers" => {
                    authors.sort_by(|a, b| b.followers().cmp(&a.followers()));
                }
                "publications" => {
                    authors.sort_by(|a, b| b.publications().cmp(&a.publications()));
                }
                "newest" => {
                    authors.reverse();
                }
                _ => {}
            }
        }

        let mut author_dtos = Vec::new();

        // TODO: improve this, please...
        for author in authors.into_iter() {
            if author.username().starts_with("admin")
                || author.username().starts_with("content-manager")
            {
                continue;
            }

            author_dtos.push(AuthorDto::from(&author));
        }

        Ok(SearchResponse {
            authors: author_dtos,
        })
    }
}
