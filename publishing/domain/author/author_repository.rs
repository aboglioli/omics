use common::error::Error;

use crate::domain::author::{Author, AuthorID};

pub trait AuthorRepository {
    fn find_by_id(&self, id: &AuthorID) -> Result<Author, Error>;
    fn save(&self, author: &mut Author) -> Result<(), Error>;
}
