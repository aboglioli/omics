use std::collections::HashMap;

use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use serde::Serialize;

use common::error::{Error, ErrorKind};

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

impl std::fmt::Display for PublicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for PublicError {}

impl From<Error> for PublicError {
    fn from(err: Error) -> Self {
        if let ErrorKind::Internal = err.kind() {
            return PublicError {
                kind: ErrorKind::Application.to_string(),
                code: "internal_server".to_owned(),
                path: "error".to_owned(),
                status: Some(500),
                message: None,
                context: HashMap::new(),
                cause: None,
            };
        }

        let cause = match err.cause() {
            Some(err) => {
                if let ErrorKind::Application = err.kind() {
                    Some(Box::new(Self::from(err.clone())))
                } else {
                    None
                }
            }
            _ => None,
        };

        PublicError {
            kind: err.kind().to_string(),
            code: err.code().to_string(),
            path: err.path().to_string(),
            status: err.status(),
            message: err.message().cloned(),
            context: err.context().clone(),
            cause,
        }
    }
}

impl error::ResponseError for PublicError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(self).unwrap())
    }
    fn status_code(&self) -> StatusCode {
        match self.status {
            Some(status) => {
                if let Ok(status) = StatusCode::from_u16(status as u16) {
                    status
                } else {
                    StatusCode::BAD_REQUEST
                }
            }
            None => StatusCode::BAD_REQUEST,
        }
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
        Error::new("one", "one")
            .set_message("message")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .wrap(
                Error::new("two", "two")
                    .add_context("prop1", "invalid")
                    .wrap(Error::internal("three", "three").wrap(
                        Error::new("four", "prop2_invalid").wrap(
                            Error::internal("five", "five").wrap_raw(StringError {
                                error: "INSERT failed".to_owned(),
                            }),
                        ),
                    )),
            )
    }

    #[test]
    fn without_internal_errors() {
        let err = err();

        let public_err = PublicError::from(err);
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
