use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Validation {
    code: String,
}

impl Validation {
    pub fn new() -> Self {
        let code = Uuid::new_v4();
        Validation {
            code: code.to_string(),
        }
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
