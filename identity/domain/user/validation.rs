use uuid::Uuid;

use common::config::Config;

#[derive(Default, Debug, Clone)]
pub struct Validation {
    code: String,
}

impl Validation {
    pub fn new() -> Self {
        let config = Config::get();

        let code = if config.env() == "development" {
            "magic".to_owned()
        } else {
            Uuid::new_v4().to_string()
        };

        Validation { code }
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl PartialEq for Validation {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl<S: Into<String>> From<S> for Validation {
    fn from(s: S) -> Self {
        Validation { code: s.into() }
    }
}
