use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio_postgres::row::Row;
use tokio_postgres::Client;
use uuid::Uuid;

use common::error::Error;
use common::model::AggregateRoot;
use common::result::Result;
use identity::domain::user::UserId;

use crate::domain::notification::{Body, Notification, NotificationId, NotificationRepository};

impl Notification {
    fn from_row(row: Row) -> Result<Notification> {
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let code: String = row.get("code");
        let body: Body = serde_json::from_value(row.get("body"))?;
        let read: bool = row.get("read");
        let datetime: DateTime<Utc> = row.get("datetime");

        Ok(Notification::build(
            AggregateRoot::build(NotificationId::new(id.to_string())?, datetime, None, None),
            UserId::new(user_id.to_string())?,
            code,
            body,
            read,
        ))
    }
}

pub struct PostgresNotificationRepository {
    client: Arc<Client>,
}

impl PostgresNotificationRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresNotificationRepository { client }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn find_by_user_id(&self, id: &UserId, _read: Option<bool>) -> Result<Vec<Notification>> {
        let rows = self
            .client
            .query(
                "SELET * FROM notifications
                WHERE user_id = $1",
                &[&id.to_uuid()?],
            )
            .await
            .map_err(|_err| Error::not_found("notification"))?;

        let mut notifications = Vec::new();
        for row in rows.into_iter() {
            notifications.push(Notification::from_row(row)?);
        }

        Ok(notifications)
    }

    async fn save(&self, notification: &mut Notification) -> Result<()> {
        let create = self
            .client
            .query_one(
                "SELECT * FROM notifications
                WHERE id = $1",
                &[&notification.base().id().to_uuid()?],
            )
            .await
            .is_err();

        let body = serde_json::to_value(notification.body())?;

        if create {
            self.client
                .execute(
                    "INSERT INTO notifications(
                        id,
                        user_id,
                        code,
                        body,
                        read,
                        datetime
                    ) VALUES (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5,
                        $6
                    )",
                    &[
                        &notification.base().id().to_uuid()?,
                        &notification.user_id().to_uuid()?,
                        &notification.code(),
                        &body,
                        &notification.is_read(),
                        &notification.base().created_at(),
                    ],
                )
                .await
                .map_err(|err| Error::new("notification", "create").wrap_raw(err))?;
        } else {
            self.client
                .execute(
                    "UPDATE notifications
                    SET
                        read = $2
                    WHERE
                        id = $1",
                    &[
                        &notification.base().id().to_uuid()?,
                        &notification.is_read(),
                    ],
                )
                .await
                .map_err(|err| Error::new("notification", "update").wrap_raw(err))?;
        }

        Ok(())
    }
}
