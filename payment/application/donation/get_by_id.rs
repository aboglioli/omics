use common::error::Error;
use common::request::Include;
use common::result::Result;
use identity::UserIdAndRole;
use publishing::application::dtos::{AuthorDto, ReaderDto};
use publishing::domain::author::AuthorRepository;
use publishing::domain::reader::ReaderRepository;

use crate::application::dtos::DonationDto;
use crate::domain::donation::{DonationId, DonationRepository};

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    donation_repo: &'a dyn DonationRepository,
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        donation_repo: &'a dyn DonationRepository,
        reader_repo: &'a dyn ReaderRepository,
    ) -> Self {
        GetById {
            author_repo,
            donation_repo,
            reader_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        donation_id: String,
        include: Include,
    ) -> Result<DonationDto> {
        let donation = self
            .donation_repo
            .find_by_id(&DonationId::new(donation_id)?)
            .await?;
        if !auth_role.can("get_any_donation") {
            if (&auth_id != donation.author_id() && &auth_id != donation.reader_id())
                || !auth_role.can("get_own_donation")
            {
                return Err(Error::unauthorized());
            }
        }

        let mut donation_dto = DonationDto::from(&donation);

        if include.has("author") {
            let author = self.author_repo.find_by_id(donation.author_id()).await?;
            donation_dto = donation_dto.author(AuthorDto::from(&author));
        }

        if include.has("reader") {
            let reader = self.reader_repo.find_by_id(donation.reader_id()).await?;
            donation_dto = donation_dto.reader(ReaderDto::from(&reader));
        }

        Ok(donation_dto)
    }
}
