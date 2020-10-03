use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    amount: f64,
}

impl Amount {
    pub fn new(amount: f64) -> Result<Self> {
        if amount < 0.0 {
            return Err(Error::new("amount", "invalid_range"));
        }

        Ok(Amount { amount })
    }

    pub fn value(&self) -> f64 {
        self.amount
    }
}
