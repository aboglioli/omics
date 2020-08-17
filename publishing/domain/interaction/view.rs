use common::result::Result;

use crate::domain::interaction::Base;
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

#[derive(Debug, Clone)]
pub struct View {
    base: Base,
    unique: bool,
}

impl View {
    pub fn new(reader_id: ReaderId, publication_id: PublicationId, unique: bool) -> Result<Self> {
        Ok(View {
            base: Base::new(reader_id, publication_id)?,
            unique,
        })
    }

    pub fn base(&self) -> &Base {
        &self.base
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }
}
