use std::str::FromStr;

use chrono::DateTime;
use serde::Deserialize;

use common::error::Error;
use common::request::{Include, PaginationParams, PaginationResponse};
use common::result::Result;
use identity::domain::user::UserId;
use identity::UserIdAndRole;
use publishing::application::dtos::{AuthorDto, ReaderDto};
use publishing::domain::author::AuthorRepository;
use publishing::domain::reader::ReaderRepository;

use crate::application::dtos::DonationDto;
use crate::domain::donation::{DonationOrderBy, DonationRepository, Status};

#[derive(Deserialize)]
pub struct SearchCommand {
    pub author_id: Option<String>,
    pub reader_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub struct Search<'a> {
    author_repo: &'a dyn AuthorRepository,
    donation_repo: &'a dyn DonationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> Search<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        donation_repo: &'a dyn DonationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        Search {
            author_repo,
            donation_repo,
            reader_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        cmd: SearchCommand,
        include: Include,
        pagination: PaginationParams,
    ) -> Result<PaginationResponse<DonationDto>> {
        if !auth_role.can("get_any_donation") {
            if let Some(author_id) = &cmd.author_id {
                if author_id != auth_id.value() || !auth_role.can("donate") {
                    return Err(Error::unauthorized());
                }
            }

            if let Some(reader_id) = &cmd.reader_id {
                if reader_id != auth_id.value() || !auth_role.can("donate") {
                    return Err(Error::unauthorized());
                }
            }
        }

        let pagination_donations = self
            .donation_repo
            .search(
                cmd.author_id.map(UserId::new).transpose()?.as_ref(),
                cmd.reader_id.map(UserId::new).transpose()?.as_ref(),
                cmd.status
                    .map(|s| Status::from_str(&s))
                    .transpose()?
                    .as_ref(),
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
                    .map(|o| DonationOrderBy::from_str(&o))
                    .transpose()?
                    .as_ref(),
            )
            .await?;

        let mut res = PaginationResponse::new(
            pagination_donations.offset(),
            pagination_donations.limit(),
            pagination_donations.total(),
            pagination_donations.matching_criteria(),
        );

        for donation in pagination_donations.into_items().into_iter() {
            let mut donation_dto = DonationDto::from(&donation);

            if include.has("author") {
                let author = self.author_repo.find_by_id(donation.author_id()).await?;
                donation_dto = donation_dto.author(AuthorDto::from(&author));
            }

            if include.has("reader") {
                let reader = self.reader_repo.find_by_id(donation.reader_id()).await?;
                donation_dto = donation_dto.reader(ReaderDto::from(&reader));
            }

            res.add_item(donation_dto);
        }

        Ok(res)
    }
}
