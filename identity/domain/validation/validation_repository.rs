use common::error::Error;

use crate::domain::validation::{Validation, ValidationCode};

pub trait ValidationRepository {
    fn find_by_code(&self, code: &ValidationCode) -> Result<Validation, Error>;
    fn save(&self, validation: &mut Validation) -> Result<(), Error>;
}
