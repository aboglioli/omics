use serde::Serialize;
use serde_json::Value;

use crate::domain::notification::Notification;

#[derive(Serialize)]
pub struct NotificationDto {
    id: String,
    user_id: String,
    code: String,
    body: Value,
    read: bool,
}

impl From<&Notification> for NotificationDto {
    fn from(notification: &Notification) -> Self {
        let body = serde_json::to_value(notification.body()).unwrap();

        NotificationDto {
            id: notification.base().id().to_string(),
            user_id: notification.user_id().to_string(),
            code: notification.code().to_string(),
            body,
            read: notification.is_read(),
        }
    }
}
