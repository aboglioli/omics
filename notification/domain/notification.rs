mod body;
mod repository;
pub use body::*;
pub use repository::*;

use common::model::{AggregateRoot, StringId};

pub type NotificationId = StringId;

#[derive(Debug, Clone)]
pub struct Notification {
    base: AggregateRoot<NotificationId>,
    body: Body,
    read: bool,
}

impl Notification {
    pub fn new<S: Into<String>>(id: NotificationId, body: Body) -> Self {
        Notification {
            base: AggregateRoot::new(id),
            read: false,
            body,
        }
    }

    pub fn build(base: AggregateRoot<NotificationId>, body: Body, read: bool) -> Self {
        Notification { base, body, read }
    }

    pub fn base(&self) -> &AggregateRoot<NotificationId> {
        &self.base
    }

    pub fn mark_as_read(&mut self) {
        self.read = true;
    }
}
