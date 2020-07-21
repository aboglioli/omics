use common::error::Error;

#[derive(Debug, Clone)]
pub struct Username {
    username: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Username, Error> {
        if username.len() < 4 {
            return Err(Error::application()
                .add_context("username", "too_short")
                .build());
        }

        if username.len() > 24 {
            return Err(Error::application()
                .add_context("username", "too_long")
                .build());
        }

        Ok(Username {
            username: String::from(username),
        })
    }

    pub fn value(&self) -> &str {
        &self.username
    }
}
