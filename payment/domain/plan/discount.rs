use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Discount {
    discount: f64,
}

impl Discount {
    pub fn new(discount: f64) -> Result<Self> {
        if discount < 0.0 || discount > 1.0 {
            return Err(Error::new("discount", "invalid_range"));
        }

        Ok(Discount { discount })
    }

    pub fn value(&self) -> f64 {
        self.discount
    }
}
