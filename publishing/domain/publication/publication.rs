use crate::common::error::Error;
use crate::common::model::ID;
use crate::publishing::domain::publication::Name;

pub type PublicationID = String;

struct Publication {
    id: ID<PublicationID>,
    name: Name,
}

impl Publication {
    fn new(id: PublicationID, name: &str) -> Result<Publication, Error> {
        Ok(Publication {
            id: ID::new(id),
            name: Name::new(name)?,
        })
    }
}
