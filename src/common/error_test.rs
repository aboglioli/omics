use std::error;
use std::fmt;

use super::error::*;

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
        .clone();
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
