use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};


use tokio_postgres::Client;
use uuid::Uuid;

use crate::error::Error;
use crate::event::{Event, EventId, EventRepository};
use crate::result::Result;
use crate::sql::where_builder::WhereBuilder;

pub struct PostgresEventRepository {
    client: Arc<Client>,
}

impl PostgresEventRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgresEventRepository { client }
    }
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
    async fn search(
        &self,
        after_id: Option<&EventId>,
        topic: Option<&String>,
        code: Option<&String>,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Event>> {
        let _after_id = after_id.map(|id| id.to_uuid()).transpose()?;

        let (sql, params) = WhereBuilder::new()
            .add_param_opt("topic = $$", &topic, topic.is_some())
            .add_param_opt("code = $$", &code, code.is_some())
            .add_param_opt("timestamp >= $$", &from, from.is_some())
            .add_param_opt("timestamp <= $$", &to, to.is_some())
            .build();

        let rows = self
            .client
            .query(&format!("SELECT * FROM events {}", sql) as &str, &params)
            .await
            .map_err(|err| Error::not_found("event").wrap_raw(err))?;

        let mut events = Vec::new();

        for row in rows.into_iter() {
            let id: Uuid = row.get("id");
            let topic: String = row.get("topic");
            let code: String = row.get("code");
            let timestamp: DateTime<Utc> = row.get("timestamp");
            let payload: Vec<u8> = serde_json::from_value(row.get("payload"))?;

            events.push(Event::build(
                EventId::new(id.to_string())?,
                topic,
                code,
                timestamp,
                payload,
            ));
        }

        Ok(events)
    }

    async fn save(&self, event: &Event) -> Result<()> {
        let payload = serde_json::to_value(event.payload())?;

        self.client
            .execute(
                "INSERT INTO events (
                    id,
                    topic,
                    code,
                    timestamp,
                    payload
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5
                )",
                &[
                    &event.id().to_uuid()?,
                    &event.topic(),
                    &event.code(),
                    &event.timestamp(),
                    &payload,
                ],
            )
            .await
            .map_err(|err| Error::new("event", "create").wrap_raw(err))?;

        Ok(())
    }
}
