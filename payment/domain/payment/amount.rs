use common::error::Error;
use common::result::Result;

pub struct Amount {
    amount: f64,
}

impl Amount {
    pub fn new(amount: f64) -> Result<Amount> {
        if amount < 0.0 {
            return Err(Error::application());
        }

        Ok(Amount { amount })
    }

    pub fn value(&self) -> f64 {
        self.amount
    }
}
