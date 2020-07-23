use common::error::Error;

pub struct Amount {
    amount: f64,
}

impl Amount {
    pub fn new(amount: f64) -> Result<Amount, Error> {
        if amount < 0.0 {
            return Err(Error::application());
        }

        Ok(Amount { amount })
    }
}
