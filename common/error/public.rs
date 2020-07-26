use std::collections::HashMap;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct PublicError {
    kind: ErrorKind,
    code: Option<String>,
    path: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    context: HashMap<String, String>,
    cause: Option<Box<PublicError>>,
}

impl PublicError {
    pub fn from(err: &Error, include_internal: bool) -> Option<PublicError> {
        let cause = match err.cause() {
            Some(err) => match err.kind() {
                ErrorKind::Internal if include_internal => {
                    match PublicError::from(err, include_internal) {
                        Some(pub_err) => Some(Box::new(pub_err)),
                        _ => None,
                    }
                }
                ErrorKind::Application => match PublicError::from(err, include_internal) {
                    Some(pub_err) => Some(Box::new(pub_err)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };

        Some(PublicError {
            kind: err.kind().clone(),
            code: err.code().cloned(),
            path: err.path().cloned(),
            status: err.status().cloned(),
            message: err.message().cloned(),
            context: err.context().clone(),
            cause,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;
    use std::fmt;

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

    fn err() -> Error {
        Error::application()
            .set_code("one")
            .set_message("message")
            .set_path("my.path")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .wrap(
                Error::application()
                    .set_code("two")
                    .add_context("prop1", "invalid")
                    .wrap(
                        Error::internal()
                            .set_code("three")
                            .wrap(
                                Error::pair("prop2", "invalid")
                                    .set_code("four")
                                    .wrap(
                                        Error::internal()
                                            .set_code("five")
                                            .wrap_raw(StringError {
                                                error: "INSERT failed".to_owned(),
                                            })
                                            .build(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build()
    }

    #[test]
    fn with_internal_errors() {
        let err = err();

        let public_err = PublicError::from(&err, true).unwrap();
        assert_eq!(public_err.kind, ErrorKind::Application);
        assert_eq!(public_err.code.unwrap(), "one");

        let two = public_err.cause.unwrap();
        assert_eq!(two.kind, ErrorKind::Application);
        assert_eq!(two.code.unwrap(), "two");

        let three = two.cause.unwrap();
        assert_eq!(three.kind, ErrorKind::Internal);
        assert_eq!(three.code.unwrap(), "three");

        let four = three.cause.unwrap();
        assert_eq!(four.kind, ErrorKind::Application);
        assert_eq!(four.code.unwrap(), "four");

        let five = four.cause.unwrap();
        assert_eq!(five.kind, ErrorKind::Internal);
        assert_eq!(five.code.unwrap(), "five");

        let six = five.cause.unwrap();
        assert_eq!(six.kind, ErrorKind::Internal);
        assert!(six.code.is_none());
        assert_eq!(six.message.unwrap(), "INSERT failed");
    }

    #[test]
    fn without_internal_errors() {
        let err = err();

        let public_err = PublicError::from(&err, false).unwrap();
        assert_eq!(public_err.kind, ErrorKind::Application);
        assert_eq!(public_err.code.unwrap(), "one");

        let two = public_err.cause.unwrap();
        assert_eq!(two.kind, ErrorKind::Application);
        assert_eq!(two.code.unwrap(), "two");
        assert!(two.cause.is_none());
    }
}
