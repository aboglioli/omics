use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Days {
    days: u32,
}

impl Days {
    pub fn new(days: u32) -> Result<Self> {
        if days < 1 {
            return Err(Error::new("days", "invalid_range"));
        }

        Ok(Days { days })
    }

    pub fn value(&self) -> u32 {
        self.days
    }
}
