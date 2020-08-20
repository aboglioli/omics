use std::collections::HashMap;

use serde::Serialize;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone, Serialize)]
pub struct PublicError {
    kind: String,
    path: String,
    code: String,
    status: Option<u32>,
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
            kind: err.kind().to_string(),
            code: err.code().to_string(),
            path: err.path().to_string(),
            status: err.status(),
            message: err.message().cloned(),
            context: err.context().clone(),
            cause,
        })
    }
}

impl warp::reject::Reject for PublicError {}

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
        Error::new("one", "one")
            .set_message("message")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .wrap(
                Error::new("two", "two")
                    .add_context("prop1", "invalid")
                    .wrap(
                        Error::internal("three", "three")
                            .wrap(
                                Error::new("four", "prop2_invalid")
                                    .wrap(
                                        Error::internal("five", "five")
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
        assert_eq!(public_err.kind, "application");
        assert_eq!(public_err.context.len(), 2);
        assert_eq!(public_err.path, "one");
        assert_eq!(public_err.code, "one");

        let two = public_err.cause.unwrap();
        assert_eq!(two.kind, "application");
        assert_eq!(two.context.len(), 1);
        assert_eq!(two.path, "two");
        assert_eq!(two.code, "two");

        let three = two.cause.unwrap();
        assert_eq!(three.kind, "internal");
        assert_eq!(three.path, "three");
        assert_eq!(three.code, "three");

        let four = three.cause.unwrap();
        assert_eq!(four.kind, "application");
        assert_eq!(four.path, "four");
        assert_eq!(four.code, "prop2_invalid");

        let five = four.cause.unwrap();
        assert_eq!(five.kind, "internal");
        assert_eq!(five.path, "five");
        assert_eq!(five.code, "five");

        let six = five.cause.unwrap();
        assert_eq!(six.kind, "internal");
        assert_eq!(six.code, "raw");
        assert_eq!(six.message.unwrap(), "INSERT failed");
    }

    #[test]
    fn without_internal_errors() {
        let err = err();

        let public_err = PublicError::from(&err, false).unwrap();
        assert_eq!(public_err.kind, "application");
        assert_eq!(public_err.path, "one");
        assert_eq!(public_err.code, "one");

        let two = public_err.cause.unwrap();
        assert_eq!(two.kind, "application");
        assert_eq!(two.path, "two");
        assert_eq!(two.code, "two");
        assert!(two.cause.is_none());
    }
}
