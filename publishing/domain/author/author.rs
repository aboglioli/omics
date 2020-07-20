use common::error::Error;
use common::model::{Entity, ID};

pub type AuthorID = String;

pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Result<Name, Error> {
        Ok(Name {
            name: name.to_owned(),
        })
    }
}

pub struct Author {
    id: ID<AuthorID>,
    name: Name,
}

impl Author {
    pub fn new(id: AuthorID, name: &str) -> Result<Author, Error> {
        Ok(Author {
            id: ID::new(id),
            name: Name::new(name)?,
        })
    }
}

impl Entity<AuthorID> for Author {
    fn id(&self) -> &ID<AuthorID> {
        &self.id
    }
}
