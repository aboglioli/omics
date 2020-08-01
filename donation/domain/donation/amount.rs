use common::result::Result;

pub struct Amount {
    amount: f64,
}

impl Amount {
    pub fn new(amount: f64) -> Result<Amount> {
        Ok(Amount { amount })
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}
