mod encoder;
mod repository;
mod service;
pub use encoder::*;
pub use repository::*;
pub use service::*;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TokenId
#[derive(Default, Debug, Clone, Eq)]
pub struct TokenId {
    id: String,
}

impl TokenId {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        TokenId { id: uuid }
    }

    pub fn build<S: Into<String>>(id: S) -> Self {
        TokenId { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl PartialEq for TokenId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TokenId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<S: Into<String>> From<S> for TokenId {
    fn from(s: S) -> TokenId {
        TokenId { id: s.into() }
    }
}

impl ToString for TokenId {
    fn to_string(&self) -> String {
        self.id().to_owned()
    }
}

// Token
#[derive(Debug, Clone)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new<S: Into<String>>(token: S) -> Token {
        Token {
            token: token.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.token
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.value().to_owned()
    }
}

// Data
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    data: HashMap<String, String>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            data: HashMap::new(),
        }
    }

    pub fn add<S: Into<String>>(&mut self, k: S, v: S) {
        self.data.insert(k.into(), v.into());
    }

    pub fn get<S: Into<String>>(&self, k: S) -> Option<&String> {
        self.data.get(&k.into())
    }
}
