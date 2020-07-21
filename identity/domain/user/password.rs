use common::error::Error;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
}

impl Password {
    pub fn new(password: &str) -> Result<Password, Error> {
        if password.len() < 50 {
            return Err(Error::application()
                .add_context("password", "not_hashed")
                .build());
        }

        Ok(Password {
            password: String::from(password),
        })
    }

    pub fn value(&self) -> &str {
        &self.password
    }
}
