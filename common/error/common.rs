use crate::error::Error;

impl Error {
    pub fn not_found<S: Into<String>>(entity: S) -> Error {
        Error::new(entity.into(), "not_found".to_owned())
            .set_status(404)
            .build()
    }

    pub fn unauthorized() -> Error {
        Error::new("authorization", "unauthorized")
            .set_status(401)
            .build()
    }
}
