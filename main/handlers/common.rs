use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Uninmplemented {
    message: String,
}

impl Uninmplemented {
    pub fn new() -> Uninmplemented {
        Uninmplemented {
            message: "unimplemented".to_owned(),
        }
    }
}
