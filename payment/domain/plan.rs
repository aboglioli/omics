mod days;
mod discount;
mod price;
pub use days::*;
pub use discount::*;
pub use price::*;

use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

pub type PlanId = StringId;

#[derive(Debug, Clone)]
pub struct Plan {
    base: AggregateRoot<PlanId, Event>,
    price: Price,
    days: Days,
    discount: Option<Discount>,
}

impl Plan {
    pub fn new(id: PlanId, price: Price, days: Days) -> Result<Self> {
        Ok(Plan {
            base: AggregateRoot::new(id),
            price,
            days,
            discount: None,
        })
    }

    pub fn base(&self) -> &AggregateRoot<PlanId, Event> {
        &self.base
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn days(&self) -> &Days {
        &self.days
    }

    pub fn discount(&self) -> Option<&Discount> {
        self.discount.as_ref()
    }

    pub fn set_discount(&mut self, discount: Discount) {
        self.discount = Some(discount);
    }
}
