use common::error::Error;
use common::result::Result;
use identity::UserIdAndRole;

use crate::application::dtos::ReaderDto;
use crate::domain::reader::ReaderRepository;

pub struct GetById<'a> {
    reader_repo: &'a dyn ReaderRepository,
}

impl<'a> GetById<'a> {
    pub fn new(reader_repo: &'a dyn ReaderRepository) -> Self {
        GetById { reader_repo }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        reader_id: String,
    ) -> Result<ReaderDto> {
        if auth_id.value() != reader_id || !auth_role.can("get_reader") {
            return Err(Error::unauthorized());
        }

        let reader = self.reader_repo.find_by_id(&auth_id).await?;

        Ok(ReaderDto::from(&reader).preferences(&reader))
    }
}
