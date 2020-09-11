use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::cache::Cache;

use crate::event::{Event, EventId, EventRepository};
use crate::infrastructure::cache::InMemCache;
use crate::result::Result;

pub struct InMemEventRepository {
    cache: InMemCache<EventId, Event>,
}

impl InMemEventRepository {
    pub fn new() -> Self {
        InMemEventRepository {
            cache: InMemCache::new(),
        }
    }
}

impl Default for InMemEventRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventRepository for InMemEventRepository {
    async fn find_all(&self) -> Result<Vec<Event>> {
        let mut events = self.cache.all().await;
        events.sort_by(|a, b| a.timestamp().cmp(b.timestamp()));
        Ok(events)
    }

    async fn find_after_id(&self, id: &EventId) -> Result<Vec<Event>> {
        let mut res = Vec::new();
        let mut store = false;
        for event in self.find_all().await?.iter() {
            if store {
                res.push(event.clone());
                continue;
            }

            if event.id() == id {
                store = true;
            }
        }

        Ok(res)
    }

    async fn find_from_date(
        &self,
        from: Option<&DateTime<Utc>>,
        to: Option<&DateTime<Utc>>,
    ) -> Result<Vec<Event>> {
        let mut events = self.find_all().await?;
        events.retain(|event| {
            let from = if let Some(from) = from {
                event.timestamp() >= &from
            } else {
                true
            };

            let to = if let Some(to) = to {
                event.timestamp() <= &to
            } else {
                true
            };

            from && to
        });

        Ok(events)
    }

    async fn save(&self, event: &Event) -> Result<()> {
        self.cache.set(event.id().clone(), event.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::TimeZone;

    fn mock_events() -> Vec<Event> {
        vec![
            Event::build(
                EventId::new("event-1").unwrap(),
                "topic".to_owned(),
                "code".to_owned(),
                Utc.ymd(2020, 08, 05).and_hms(15, 30, 0),
                vec![1, 2, 3],
            ),
            Event::build(
                EventId::new("event-2").unwrap(),
                "topic".to_owned(),
                "code".to_owned(),
                Utc.ymd(2020, 08, 05).and_hms(12, 45, 0),
                vec![1, 2, 3],
            ),
            Event::build(
                EventId::new("event-3").unwrap(),
                "topic".to_owned(),
                "code".to_owned(),
                Utc.ymd(2020, 08, 05).and_hms(16, 15, 0),
                vec![1, 2, 3],
            ),
            Event::build(
                EventId::new("event-4").unwrap(),
                "topic".to_owned(),
                "code".to_owned(),
                Utc.ymd(2020, 08, 04).and_hms(19, 0, 0),
                vec![1, 2, 3],
            ),
            Event::build(
                EventId::new("event-5").unwrap(),
                "topic".to_owned(),
                "code".to_owned(),
                Utc.ymd(2020, 08, 06).and_hms(19, 0, 0),
                vec![1, 2, 3],
            ),
        ]
    }

    #[tokio::test]
    async fn find_and_filter() {
        let repo = InMemEventRepository::new();
        let mut events = mock_events();
        for mut event in events.iter_mut() {
            repo.save(&mut event).await.unwrap();
        }

        let events = repo.find_all().await.unwrap();
        assert_eq!(events[0].id().value(), "event-4");
        assert_eq!(events[1].id().value(), "event-2");
        assert_eq!(events[2].id().value(), "event-1");
        assert_eq!(events[3].id().value(), "event-3");
        assert_eq!(events[4].id().value(), "event-5");

        let events = repo
            .find_after_id(&EventId::new("event-1").unwrap())
            .await
            .unwrap();
        assert_eq!(events[0].id().value(), "event-3");
        assert_eq!(events[1].id().value(), "event-5");

        let events = repo
            .find_from_date(
                Some(&Utc.ymd(2020, 08, 05).and_hms(15, 0, 0)),
                Some(&Utc.ymd(2020, 08, 06).and_hms(0, 0, 0)),
            )
            .await
            .unwrap();
        assert_eq!(events[0].id().value(), "event-1");
        assert_eq!(events[1].id().value(), "event-3");
    }
}
