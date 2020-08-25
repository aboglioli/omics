use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Price {
    price: f64,
}

impl Price {
    pub fn new(price: f64) -> Result<Self> {
        if price < 0.0 {
            return Err(Error::new("price", "negative"));
        }

        Ok(Price { price })
    }

    pub fn value(&self) -> f64 {
        self.price
    }
}
