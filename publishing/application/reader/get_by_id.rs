use common::error::Error;
use common::result::Result;
use identity::domain::user::UserRepository;

use crate::application::dtos::ReaderDto;
use crate::domain::reader::{ReaderId, ReaderRepository};

pub struct GetById<'a> {
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> GetById<'a> {
    pub fn new(reader_repo: &'a dyn ReaderRepository) -> Self {
        GetById { reader_repo }
    }

    pub async fn exec(&self, auth_id: String, reader_id: String) -> Result<ReaderDto> {
        if auth_id != reader_id {
            return Err(Error::unauthorized());
        }

        let reader_id = ReaderId::new(auth_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        Ok(ReaderDto::from(&reader).preferences(&reader))
    }
}
