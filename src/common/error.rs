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
        let err = Error::internal().set_message(&err.to_string()).clone();
        self.cause = Some(Box::new(err));
        self
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
