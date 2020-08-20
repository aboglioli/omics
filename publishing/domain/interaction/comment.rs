use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
    pub fn new<S: Into<String>>(comment: S) -> Result<Self> {
        let comment = comment.into();

        if comment.len() < 4 {
            return Err(Error::new("comment", "too_short"));
        }

        Ok(Comment { comment })
    }

    pub fn value(&self) -> &str {
        &self.comment
    }
}

impl ToString for Comment {
    fn to_string(&self) -> String {
        self.value().to_owned()
    }
}
