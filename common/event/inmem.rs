use std::sync::Arc;

use async_trait::async_trait;
use regex::Regex;
use tokio::sync::Mutex;

use crate::error::Error;
use crate::event::{Event, EventHandler, EventPublisher, EventSubscriber};
use crate::result::Result;

pub struct InMemEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<Output = bool> + Sync>>>>,
}

impl InMemEventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl EventPublisher for InMemEventBus {
    type Output = bool;

    async fn publish(&self, event: Event) -> Result<Self::Output> {
        let mut handlers = self.handlers.lock().await;
        for handler in handlers.iter_mut() {
            if let Ok(re) = Regex::new(handler.topic()) {
                if re.is_match(event.topic()) {
                    if let Err(err) = handler.handle(&event).await {
                        return Err(Error::internal().wrap(err).build());
                    }
                }
            } else {
                println!("invalid regex");
            }
        }

        Ok(true)
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output> {
        for event in events.into_iter() {
            self.publish(event).await?;
        }

        Ok(true)
    }
}

#[async_trait]
impl EventSubscriber for InMemEventBus {
    type Output = bool;

    async fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output> + Sync>,
    ) -> Result<Self::Output> {
        let mut handlers = self.handlers.lock().await;
        handlers.push(handler);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use crate::mocks::Counter;

    fn create_event(topic: &str) -> Event {
        Event::new(topic, topic, topic.as_bytes().to_vec())
    }

    struct BasicHandler {
        counter: Arc<Counter>,
        topic: String,
    }

    impl BasicHandler {
        fn new(topic: &str) -> Self {
            BasicHandler {
                counter: Arc::new(Counter::new()),
                topic: topic.to_owned(),
            }
        }

        pub fn counter(&self) -> &Counter {
            &self.counter
        }

        pub fn clone_with_topic(&self, topic: &str) -> Self {
            BasicHandler {
                counter: Arc::clone(&self.counter),
                topic: topic.to_owned(),
            }
        }

        pub fn clone(&self) -> Self {
            self.clone_with_topic(&self.topic)
        }
    }

    #[async_trait::async_trait]
    impl EventHandler for BasicHandler {
        type Output = bool;

        fn topic(&self) -> &str {
            &self.topic
        }

        async fn handle(&mut self, event: &Event) -> Result<Self::Output> {
            self.counter.inc(event.topic());
            Ok(true)
        }
    }

    #[tokio::test]
    async fn create() {
        let eb = InMemEventBus::new();
        assert_eq!(eb.handlers.lock().await.len(), 0);
    }

    #[tokio::test]
    async fn polymorphic() {
        let _: Box<dyn EventSubscriber<Output = _>> = Box::new(InMemEventBus::new());
        let _: Box<dyn EventPublisher<Output = _>> = Box::new(InMemEventBus::new());

        let mut eb = InMemEventBus::new();
        let subscriber: &mut dyn EventSubscriber<Output = _> = &mut eb;
        let handler = BasicHandler::new(".*");

        subscriber
            .subscribe(Box::new(handler.clone()))
            .await
            .unwrap();
        subscriber
            .subscribe(Box::new(handler.clone()))
            .await
            .unwrap();
        subscriber
            .subscribe(Box::new(handler.clone()))
            .await
            .unwrap();
        subscriber
            .subscribe(Box::new(handler.clone()))
            .await
            .unwrap();

        let publisher: &mut dyn EventPublisher<Output = _> = &mut eb;
        assert!(publisher.publish(create_event("evented")).await.is_ok());
        assert_eq!(handler.counter().count("evented"), 4);
    }

    #[tokio::test]
    async fn publish_subscribe() {
        let eb = InMemEventBus::new();
        let handler = BasicHandler::new("*");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.created")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.created")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.updated")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent2.created")))
            .await
            .is_ok());

        assert!(eb.publish(create_event("ent1.created")).await.is_ok());
        assert!(eb.publish(create_event("ent2.created")).await.is_ok());

        assert_eq!(handler.counter().count("ent1.created"), 2);
        assert_eq!(handler.counter().count("ent1.updated"), 0);
        assert_eq!(handler.counter().count("ent2.created"), 1);
    }

    #[tokio::test]
    async fn match_topics_with_regex() {
        let eb = InMemEventBus::new();
        let handler = BasicHandler::new(".+");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(".+")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(".+.created")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.+")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.created")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.updated")))
            .await
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.(deleted|updated)")))
            .await
            .is_ok());

        assert!(eb.publish(create_event("ent.created")).await.is_ok());
        assert!(eb.publish(create_event("ent.updated")).await.is_ok());
        assert!(eb.publish(create_event("ent.deleted")).await.is_ok());

        assert_eq!(handler.counter().count("ent.created"), 4);
        assert_eq!(handler.counter().count("ent.updated"), 4);
        assert_eq!(handler.counter().count("ent.deleted"), 3);
    }

    #[tokio::test]
    async fn publish_all() {
        let eb = InMemEventBus::new();
        let handler = BasicHandler::new(".+");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(r"ent.+|.+.created")))
            .await
            .is_ok());

        assert!(eb
            .publish_all(vec![
                create_event("another.created"),
                create_event("ent.created"),
                create_event("ent.updated"),
                create_event("ent.deleted"),
                create_event("another.updated"),
            ])
            .await
            .is_ok());

        assert_eq!(handler.counter().count("ent.created"), 1);
        assert_eq!(handler.counter().count("ent.updated"), 1);
        assert_eq!(handler.counter().count("ent.deleted"), 1);
        assert_eq!(handler.counter().count("another.created"), 1);
        assert_eq!(handler.counter().count("another.updated"), 0);
    }

    #[tokio::test]
    async fn multiple_publishers() {
        let eb = Arc::new(InMemEventBus::new());
        let h = BasicHandler::new(r"^topic[0-9]+$");
        eb.subscribe(Box::new(h.clone())).await.unwrap();

        eb.publish(create_event("topic007")).await.unwrap();

        let sub_eb = Arc::clone(&eb);
        let j1 = tokio::spawn(async move {
            sub_eb.publish(create_event("topic1")).await.unwrap();
        });

        let sub_eb = Arc::clone(&eb);
        let j2 = tokio::spawn(async move {
            sub_eb.publish(create_event("topic2")).await.unwrap();
        });

        let (_, _) = tokio::join!(j1, j2);

        assert_eq!(h.counter().count("topic007"), 1);
        assert_eq!(h.counter().count("topic1"), 1);
        assert_eq!(h.counter().count("topic2"), 1);
    }
}
