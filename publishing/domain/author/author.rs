use common::error::Error;
use common::model::AggregateRoot;

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
    base: AggregateRoot<AuthorID>,
    name: Name,
}

impl Author {
    pub fn new(id: AuthorID, name: Name) -> Result<Author, Error> {
        Ok(Author {
            base: AggregateRoot::new(id),
            name,
        })
    }
}
