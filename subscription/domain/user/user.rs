use common::error::Error;
use common::model::{AggregateRoot, DefaultEvent};

use crate::domain::user::PaymentMethod;

pub type UserId = String;

pub struct User {
    base: AggregateRoot<UserId, DefaultEvent>,
    name: String,
    payment_methods: Vec<PaymentMethod>,
}

impl User {
    pub fn new(id: UserId, name: &str) -> Result<User, Error> {
        Ok(User {
            base: AggregateRoot::new(id),
            name: name.to_owned(),
            payment_methods: Vec::new(),
        })
    }
}
