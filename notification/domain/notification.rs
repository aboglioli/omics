mod body;
mod repository;
pub use body::*;
pub use repository::*;

use common::model::{AggregateRoot, StringId};
use identity::domain::user::UserId;

pub type NotificationId = StringId;

#[derive(Debug, Clone)]
pub struct Notification {
    base: AggregateRoot<NotificationId>,
    user_id: UserId,
    code: String,
    body: Body,
    read: bool,
}

impl Notification {
    pub fn new<S: Into<String>>(id: NotificationId, user_id: UserId, code: S, body: Body) -> Self {
        Notification {
            base: AggregateRoot::new(id),
            user_id,
            code: code.into(),
            body,
            read: false,
        }
    }

    pub fn build(
        base: AggregateRoot<NotificationId>,
        user_id: UserId,
        code: String,
        body: Body,
        read: bool,
    ) -> Self {
        Notification {
            base,
            user_id,
            code,
            body,
            read,
        }
    }

    pub fn base(&self) -> &AggregateRoot<NotificationId> {
        &self.base
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn is_read(&self) -> bool {
        self.read
    }

    pub fn mark_as_read(&mut self) {
        self.read = true;
    }
}
