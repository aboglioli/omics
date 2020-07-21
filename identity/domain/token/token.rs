use std::cmp::PartialEq;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// TokenID
#[derive(Debug, Clone, Eq)]
pub struct TokenID {
    id: String,
}

impl TokenID {
    pub fn new() -> TokenID {
        TokenID {
            id: "T001".to_owned(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

impl PartialEq for TokenID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TokenID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<&str> for TokenID {
    fn from(s: &str) -> TokenID {
        TokenID { id: s.to_owned() }
    }
}

// Token
#[derive(Debug, Clone)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: &str) -> Token {
        Token {
            token: token.to_owned(),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

// Data
#[derive(Debug, Clone)]
pub struct Data {
    data: HashMap<String, String>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, k: &str, v: &str) {
        self.data.insert(k.to_owned(), v.to_owned());
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.data.get(&k.to_owned())
    }
}
