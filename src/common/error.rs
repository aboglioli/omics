use std::cmp;
use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Internal,
    Application,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    code: Option<String>,
    path: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    context: HashMap<String, String>,
    cause: Option<Box<Error>>,
}

impl Error {
    fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
            code: None,
            path: None,
            status: None,
            message: None,
            context: HashMap::new(),
            cause: None,
        }
    }

    pub fn internal() -> Error {
        Error::new(ErrorKind::Internal)
    }

    pub fn application() -> Error {
        Error::new(ErrorKind::Application)
    }

    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn context(&self) -> &HashMap<String, String> {
        &self.context
    }

    pub fn has_context(&self) -> bool {
        !self.context.is_empty()
    }

    pub fn cause(&self) -> Option<&Error> {
        match &self.cause {
            Some(boxed_err) => Some(boxed_err.as_ref()),
            None => None,
        }
    }

    pub fn set_code(&mut self, code: &str) -> &mut Error {
        self.code = Some(String::from(code));
        self
    }

    pub fn set_path(&mut self, path: &str) -> &mut Error {
        self.path = Some(String::from(path));
        self
    }

    pub fn set_status(&mut self, status: i32) -> &mut Error {
        self.status = Some(status);
        self
    }

    pub fn set_message(&mut self, message: &str) -> &mut Error {
        self.message = Some(String::from(message));
        self
    }

    pub fn add_context(&mut self, k: &str, v: &str) -> &mut Error {
        self.context.insert(String::from(k), String::from(v));
        self
    }

    pub fn wrap(&mut self, err: Error) -> &mut Error {
        self.cause = Some(Box::new(err));
        self
    }

    pub fn wrap_raw<E: error::Error>(&mut self, err: E) -> &mut Error {
        let err = Error::internal().set_message(&err.to_string()).build();
        self.cause = Some(Box::new(err));
        self
    }

    pub fn build(&self) -> Error {
        self.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.path == other.path && self.status == other.status
    }
}

#[cfg(test)]
mod tests {
    use std::error;
    use std::fmt;

    use super::*;

    #[test]
    fn basic() {
        let err = Error::internal()
            .set_code("code")
            .set_message("message")
            .set_path("my.path")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .build();
        assert_eq!(err.code().unwrap(), "code");
        assert_eq!(err.message().unwrap(), "message");
        assert_eq!(err.path().unwrap(), "my.path");
        assert_eq!(err.status().unwrap(), &404);
        assert_eq!(err.context().len(), 2);
        assert_eq!(err.context().get("k1").unwrap(), "v1");
        assert_eq!(err.context().get("k2").unwrap(), "v3");
    }

    #[derive(Debug, Clone)]
    struct StringError {
        error: String,
    }

    impl fmt::Display for StringError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.error)
        }
    }

    impl error::Error for StringError {}

    #[test]
    fn wrap() {
        let raw_err = StringError {
            error: "raw_err".to_owned(),
        };
        let inner_err = Error::internal()
            .set_code("inner")
            .wrap_raw(raw_err.clone())
            .clone();
        let outer_err = Error::application()
            .set_code("outer")
            .wrap(inner_err.clone())
            .clone();

        assert_eq!(
            inner_err.cause().unwrap().message().unwrap(),
            &raw_err.error
        );
        assert_eq!(outer_err.cause().unwrap(), &inner_err);
    }
}
