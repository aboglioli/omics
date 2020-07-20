use crate::domain::author::{Author, AuthorID};
use crate::domain::publication::{Name, Page, Statistics, Synopsis};
use common::error::Error;
use common::model::{Entity, ID};

pub type PublicationID = String;

struct Publication {
    id: ID<PublicationID>,
    name: Name,
    synopsis: Synopsis,
    author_id: AuthorID,
    statistics: Statistics,
    pages: Vec<Page>,
}

impl Publication {
    fn new(
        id: PublicationID,
        name: &str,
        synopsis: &str,
        author: &Author,
        statistics: Statistics,
    ) -> Result<Publication, Error> {
        Ok(Publication {
            id: ID::new(id),
            name: Name::new(name)?,
            synopsis: Synopsis::new(synopsis)?,
            author_id: author.id().value(),
            statistics: Statistics::new(),
            pages: Vec::new(),
        })
    }
}
