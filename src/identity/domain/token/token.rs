use std::collections::HashMap;

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

pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: &str) -> Token {
        Token {
            token: token.to_owned(),
        }
    }
}

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
