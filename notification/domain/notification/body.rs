use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Body {
    publication_id: Option<String>,
    publication_name: Option<String>,

    collection_id: Option<String>,
    collection_name: Option<String>,

    author_id: Option<String>,
    author_username: Option<String>,
    author_name: Option<String>,
    author_lastname: Option<String>,

    reader_id: Option<String>,
    reader_username: Option<String>,
    reader_name: Option<String>,
    reader_lastname: Option<String>,

    amount: Option<f64>,
}

impl Body {
    pub fn new() -> Self {
        Body::default()
    }

    pub fn publication<S: Into<String>>(mut self, publication_id: S, publication_name: S) -> Self {
        self.publication_id = Some(publication_id.into());
        self.publication_name = Some(publication_name.into());
        self
    }

    pub fn collection<S: Into<String>>(mut self, collection_id: S, collection_name: S) -> Self {
        self.collection_id = Some(collection_id.into());
        self.collection_name = Some(collection_name.into());
        self
    }

    pub fn author<S: Into<String>>(mut self, author_id: S, author_username: S) -> Self {
        self.author_id = Some(author_id.into());
        self.author_username = Some(author_username.into());
        self
    }

    pub fn author_name<S: Into<String>>(mut self, author_name: S, author_lastname: S) -> Self {
        self.author_name = Some(author_name.into());
        self.author_lastname = Some(author_lastname.into());
        self
    }

    pub fn reader<S: Into<String>>(mut self, reader_id: S, reader_username: S) -> Self {
        self.reader_id = Some(reader_id.into());
        self.reader_username = Some(reader_username.into());
        self
    }

    pub fn reader_name<S: Into<String>>(mut self, reader_name: S, reader_lastname: S) -> Self {
        self.reader_name = Some(reader_name.into());
        self.reader_lastname = Some(reader_lastname.into());
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
}
