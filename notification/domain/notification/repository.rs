use async_trait::async_trait;

use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::notification::Notification;

#[async_trait]
pub trait NotificationRepository: Sync + Send {
    async fn find_by_user_id(&self, id: &UserId, read: Option<bool>) -> Result<Vec<Notification>>;
    async fn save(&self, notification: &mut Notification) -> Result<()>;
}
