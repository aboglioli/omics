use common::error::Error;
use common::request::CommandResponse;
use common::result::Result;

use identity::UserIdAndRole;

use crate::domain::notification::NotificationRepository;

pub struct MarkAllAsRead<'a> {
    notification_repo: &'a dyn NotificationRepository,
}

impl<'a> MarkAllAsRead<'a> {
    pub fn new(notification_repo: &'a dyn NotificationRepository) -> Self {
        MarkAllAsRead { notification_repo }
    }

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole) -> Result<CommandResponse> {
        if !auth_role.can("get_notifications") {
            return Err(Error::unauthorized());
        }

        let unread_notifications = self
            .notification_repo
            .find_by_user_id(&auth_id, Some(false))
            .await?;

        for mut notification in unread_notifications.into_iter() {
            notification.mark_as_read();
            self.notification_repo.save(&mut notification).await?;
        }

        Ok(CommandResponse::default())
    }
}
