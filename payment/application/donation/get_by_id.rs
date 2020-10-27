




use common::error::Error;
use common::request::{Include};
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};
use publishing::application::dtos::{AuthorDto, ReaderDto};
use publishing::domain::author::AuthorRepository;
use publishing::domain::reader::ReaderRepository;

use crate::application::dtos::DonationDto;
use crate::domain::donation::{DonationId, DonationOrderBy, DonationRepository, Status};

pub struct GetById<'a> {
    author_repo: &'a dyn AuthorRepository,
    donation_repo: &'a dyn DonationRepository,
    reader_repo: &'a dyn ReaderRepository,
    user_repo: &'a dyn UserRepository,
}

impl<'a> GetById<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        donation_repo: &'a dyn DonationRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
    ) -> Self {
        GetById {
            author_repo,
            donation_repo,
            reader_repo,
            user_repo,
        }
    }

    pub async fn exec(
        &self,
        auth_id: String,
        donation_id: String,
        include: Include,
    ) -> Result<DonationDto> {
        let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
        let donation = self
            .donation_repo
            .find_by_id(&DonationId::new(donation_id)?)
            .await?;
        if !user.is_admin() {
            if user.base().id() != donation.author_id() && user.base().id() != donation.reader_id()
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
