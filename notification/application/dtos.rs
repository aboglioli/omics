use serde::Serialize;

use crate::domain::notification::Notification;

#[derive(Serialize)]
pub struct NotificationDto {}

impl From<&Notification> for NotificationDto {
    fn from(_notification: &Notification) -> Self {
        NotificationDto {}
    }
}
