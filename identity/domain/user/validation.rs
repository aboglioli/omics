use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Validation {
    code: String,
}

impl Validation {
    pub fn new() -> Self {
        Validation {
            code: Uuid::new_v4().to_string(),
        }
    }

    pub fn build<S: Into<String>>(code: S) -> Self {
        Validation { code: code.into() }
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
