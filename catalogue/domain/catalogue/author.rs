use common::result::Result;

#[derive(Debug, Clone)]
pub struct Author {
    id: String,
    username: String,
    name: String,
    lastname: String,
    publications: usize,
}

impl Author {
    pub fn new<S: Into<String>>(
        id: S,
        username: S,
        name: S,
        lastname: S,
        publications: usize,
    ) -> Result<Self> {
        Ok(Author {
            id: id.into(),
            username: username.into(),
            name: name.into(),
            lastname: lastname.into(),
            publications,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lastname(&self) -> &str {
        &self.lastname
    }

    pub fn publications(&self) -> usize {
        self.publications
    }
}
