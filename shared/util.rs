use serde::Serialize;

use common::error::Error;
use common::result::Result;

pub fn serialize<T: Serialize>(event: &T, entity: &str) -> Result<Vec<u8>> {
    match serde_json::to_vec(event) {
        Ok(vec) => Ok(vec),
        Err(err) => Err(Error::new(entity, "event").wrap_raw(err).build()),
    }
}
