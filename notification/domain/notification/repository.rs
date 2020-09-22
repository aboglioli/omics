use async_trait::async_trait;
use uuid::Uuid;

use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::notification::{Notification, NotificationId};

#[async_trait]
pub trait NotificationRepository: Sync + Send {
    async fn next_id(&self) -> Result<NotificationId> {
        NotificationId::new(Uuid::new_v4().to_string())
    }

    async fn find_by_user_id(&self, id: &UserId, read: Option<bool>) -> Result<Vec<Notification>>;
    async fn save(&self, notification: &mut Notification) -> Result<()>;
}
