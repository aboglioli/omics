use common::error::Error;
use common::event::BasicEvent;
use common::model::AggregateRoot;

pub type AuthorId = String;

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
    base: AggregateRoot<AuthorId, BasicEvent>,
    name: Name,
}

impl Author {
    pub fn new(id: AuthorId, name: Name) -> Result<Author, Error> {
        Ok(Author {
            base: AggregateRoot::new(id),
            name,
        })
    }
}
