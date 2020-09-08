mod repository;
pub use repository::*;

use common::error::Error;
use common::event::Event;
use common::model::{AggregateRoot, StringId};
use common::result::Result;

use crate::domain::payment::Amount;
use crate::domain::subscription::{Status, Subscription};

pub type UserId = StringId;

#[derive(Debug, Clone)]
pub struct User {
    base: AggregateRoot<UserId, Event>,
    subscriptions: Vec<Subscription>,
}

impl User {
    pub fn new(id: UserId) -> Result<Self> {
        Ok(User {
            base: AggregateRoot::new(id),
            subscriptions: Vec::new(),
        })
    }

    pub fn base(&self) -> &AggregateRoot<UserId, Event> {
        &self.base
    }

    pub fn add_subscription(&mut self, subscription: Subscription) -> Result<()> {
        if let Some(subscription) = self.current_subscription() {
            if matches!(subscription.status_history().current(), Status::Active) {
                return Err(Error::new("subscription", "already_exists"));
            }
        }

        self.subscriptions.push(subscription);

        Ok(())
    }

    pub fn subscriptions(&self) -> &[Subscription] {
        self.subscriptions.as_ref()
    }

    pub fn current_subscription(&self) -> Option<&Subscription> {
        self.subscriptions.last()
    }

    pub fn pay_subscription(&self, _amount: Amount) -> Result<()> {
        Ok(())
    }

    pub fn has_subscription(&self) -> bool {
        self.current_subscription().is_some()
    }
}
