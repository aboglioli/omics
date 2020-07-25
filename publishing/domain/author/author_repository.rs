use common::error::Error;

use crate::domain::author::{Author, AuthorId};

pub trait AuthorRepository {
    fn find_by_id(&self, id: &AuthorId) -> Result<Author, Error>;
    fn save(&self, author: &mut Author) -> Result<(), Error>;
}
