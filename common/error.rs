pub mod common;

use std::cmp;
use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Internal,
    Application,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::Internal => "internal".to_owned(),
            ErrorKind::Application => "application".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    path: String,
    code: String,
    status: Option<u32>,
    message: Option<String>,
    context: HashMap<String, String>,
    cause: Option<Box<Error>>,
}

impl Error {
    pub fn new<S: Into<String>>(path: S, code: S) -> Error {
        Error {
            kind: ErrorKind::Application,
            path: path.into(),
            code: code.into(),
            status: None,
            message: None,
            context: HashMap::new(),
            cause: None,
        }
    }

    pub fn internal<S: Into<String>>(path: S, code: S) -> Error {
        Error {
            kind: ErrorKind::Internal,
            path: path.into(),
            code: code.into(),
            status: None,
            message: None,
            context: HashMap::new(),
            cause: None,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn status(&self) -> Option<u32> {
        self.status.clone()
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

    pub fn set_path<S: Into<String>>(&mut self, path: S) -> &mut Error {
        self.path = path.into();
        self
    }

    pub fn set_code<S: Into<String>>(&mut self, code: S) -> &mut Error {
        self.code = code.into();
        self
    }

    pub fn set_status(&mut self, status: u32) -> &mut Error {
        self.status = Some(status);
        self
    }

    pub fn set_message<S: Into<String>>(&mut self, message: S) -> &mut Error {
        self.message = Some(message.into());
        self
    }

    pub fn add_context<S: Into<String>>(&mut self, k: S, v: S) -> &mut Error {
        self.context.insert(k.into(), v.into());
        self
    }

    pub fn wrap(&mut self, err: Error) -> &mut Error {
        self.cause = Some(Box::new(err));
        self
    }

    pub fn wrap_raw<E: error::Error>(&mut self, err: E) -> &mut Error {
        let err = Error {
            kind: ErrorKind::Internal,
            path: "".to_owned(),
            code: "raw".to_owned(),
            status: None,
            message: Some(err.to_string()),
            context: HashMap::new(),
            cause: None,
        };
        self.cause = Some(Box::new(err));
        self
    }

    pub fn merge(&mut self, err: Error) -> &mut Error {
        self.add_context(err.path(), err.code());
        self.context.extend(err.context);
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
        let err = Error::new("my.path", "code")
            .set_message("message")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .build();
        assert_eq!(err.code(), "code");
        assert_eq!(err.message().unwrap(), "message");
        assert_eq!(err.path(), "my.path");
        assert_eq!(err.status().unwrap(), 404);
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
        let inner_err = Error::new("inner", "inner")
            .wrap_raw(raw_err.clone())
            .build();
        let outer_err = Error::new("outer", "outer")
            .set_code("outer")
            .wrap(inner_err.build())
            .build();

        assert_eq!(
            inner_err.cause().unwrap().message().unwrap(),
            &raw_err.error
        );
        assert_eq!(outer_err.cause().unwrap(), &inner_err);
    }

    #[test]
    fn merge() {
        let mut err1 = Error::internal("err1", "err1");
        err1.add_context("e1-key1", "value1");
        err1.add_context("e1-key2", "value2");

        let mut err2 = Error::internal("err2", "err2");
        err2.add_context("e2-key", "value");
        err2.merge(err1);

        let mut err3 = Error::new("err3", "err3");
        err3.add_context("e1-key1", "value");
        err3.add_context("e3-key", "value");
        err3.merge(err2);

        assert_eq!(err3.context().len(), 6);
        assert_eq!(err3.context().get("err1"), Some(&"err1".to_owned()));
        assert_eq!(err3.context().get("err2"), Some(&"err2".to_owned()));
        assert_eq!(err3.context().get("e1-key1"), Some(&"value1".to_owned()));
        assert_eq!(err3.context().get("e1-key2"), Some(&"value2".to_owned()));
        assert_eq!(err3.context().get("e2-key"), Some(&"value".to_owned()));
        assert_eq!(err3.context().get("e3-key"), Some(&"value".to_owned()));
    }
}
