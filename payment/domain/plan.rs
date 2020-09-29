mod price;
mod repository;
pub use price::*;
pub use repository::*;

use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type PlanId = StringId;

#[derive(Debug, Clone)]
pub struct Plan {
    base: AggregateRoot<PlanId>,
    price: Price,
}

impl Plan {
    pub fn new(id: PlanId, price: Price) -> Result<Self> {
        Ok(Plan {
            base: AggregateRoot::new(id),
            price,
        })
    }

    pub fn base(&self) -> &AggregateRoot<PlanId> {
        &self.base
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}
