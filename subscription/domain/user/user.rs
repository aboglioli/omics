use common::event::Event;
use common::model::AggregateRoot;
use common::result::Result;

use crate::domain::user::PaymentMethod;

pub type UserId = String;

pub struct User {
    base: AggregateRoot<UserId, Event>,
    name: String,
    payment_methods: Vec<PaymentMethod>,
}

impl User {
    pub fn new(id: UserId, name: &str) -> Result<User> {
        Ok(User {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
            payment_methods: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<UserId, Event> {
        &self.base
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payment_methods(&self) -> &[PaymentMethod] {
        &self.payment_methods
    }
}
