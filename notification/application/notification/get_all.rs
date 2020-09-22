use serde::Serialize;

use common::result::Result;
use identity::domain::user::UserId;

use crate::application::dtos::NotificationDto;
use crate::domain::notification::NotificationRepository;

#[derive(Serialize)]
pub struct GetAllResponse {
    notifications: Vec<NotificationDto>,
}

pub struct GetAll<'a> {
    notification_repo: &'a dyn NotificationRepository,
}

impl<'a> GetAll<'a> {
    pub fn new(notification_repo: &'a dyn NotificationRepository) -> Self {
        GetAll { notification_repo }
    }

    pub async fn exec(&self, auth_id: String) -> Result<GetAllResponse> {
        let notifications = self
            .notification_repo
            .find_by_user_id(&UserId::new(auth_id)?, None)
            .await?;

        Ok(GetAllResponse {
            notifications: notifications.iter().map(NotificationDto::from).collect(),
        })
    }
}
