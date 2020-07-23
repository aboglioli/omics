use common::error::Error;

use crate::domain::publication::{Publication, PublicationID};

pub trait PublicationRepository {
    fn find_by_id(&self, id: &PublicationID) -> Result<Publication, Error>;
}
