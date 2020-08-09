use std::sync::Arc;

use async_trait::async_trait;
use regex::Regex;
use tokio::sync::oneshot::{self, Receiver};
use tokio::sync::Mutex;

use crate::error::Error;
use crate::event::{Event, EventHandler, EventPublisher, EventSubscriber};
use crate::result::Result;

async fn publish_one(
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<Output = bool> + Sync + Send>>>>,
    event: &Event,
) -> Result<bool> {
    let mut handlers = handlers.lock().await;
    for handler in handlers.iter_mut() {
        match Regex::new(handler.topic()) {
            Ok(re) => {
                if re.is_match(event.topic()) {
                    if let Err(err) = handler.handle(&event).await {
                        return Err(Error::internal("event_publisher", "handler_error")
                            .wrap(err)
                            .build());
                    }
                }
            }
            Err(err) => {
                return Err(Error::internal("event_publisher", "invalid_topic_regex")
                    .wrap_raw(err)
                    .build())
            }
        }
    }

    Ok(true)
}

pub struct InMemEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<Output = bool> + Sync + Send>>>>,
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
    type Output = Receiver<u32>;

    async fn publish(&self, event: Event) -> Result<Self::Output> {
        let handlers = Arc::clone(&self.handlers);
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let mut c = 0;
            if let Ok(_res) = publish_one(handlers, &event).await {
                c += 1;
            }
            if let Err(_) = tx.send(c) {
                println!("Closed channel");
            }
        });

        Ok(rx)
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output> {
        let handlers = Arc::clone(&self.handlers);
        let (tx, rx) = oneshot::channel();
        let mut c = 0;

        tokio::spawn(async move {
            for event in events.into_iter() {
                if let Ok(_res) = publish_one(Arc::clone(&handlers), &event).await {
                    c += 1;
                }
            }

            if let Err(_) = tx.send(c) {
                println!("Closed channel");
            }
        });

        Ok(rx)
    }
}

#[async_trait]
impl EventSubscriber for InMemEventBus {
    type Output = bool;

    async fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output> + Sync + Send>,
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

    #[async_trait]
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

    struct ErrorHandler;

    #[async_trait]
    impl EventHandler for ErrorHandler {
        type Output = bool;

        fn topic(&self) -> &str {
            "error.*"
        }

        async fn handle(&mut self, _: &Event) -> Result<Self::Output> {
            Err(Error::new("error_handler", "error"))
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
        let rx = publisher.publish(create_event("evented")).await.unwrap();
        assert_eq!(rx.await.unwrap(), 1);
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

        let rx = eb.publish(create_event("ent1.created")).await.unwrap();
        assert_eq!(rx.await.unwrap(), 1);
        let rx = eb.publish(create_event("ent2.created")).await.unwrap();
        assert_eq!(rx.await.unwrap(), 1);

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

        assert_eq!(
            eb.publish(create_event("ent.created"))
                .await
                .unwrap()
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            eb.publish(create_event("ent.updated"))
                .await
                .unwrap()
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            eb.publish(create_event("ent.deleted"))
                .await
                .unwrap()
                .await
                .unwrap(),
            1
        );

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

        let rx = eb
            .publish_all(vec![
                create_event("another.created"),
                create_event("ent.created"),
                create_event("ent.updated"),
                create_event("ent.deleted"),
                create_event("another.updated"),
            ])
            .await
            .unwrap();
        assert_eq!(rx.await.unwrap(), 5);

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

    #[tokio::test]
    async fn errors() {
        let eb = InMemEventBus::new();
        eb.subscribe(Box::new(BasicHandler::new("ok")))
            .await
            .unwrap();
        eb.subscribe(Box::new(BasicHandler::new("ok")))
            .await
            .unwrap();
        eb.subscribe(Box::new(BasicHandler::new("error")))
            .await
            .unwrap();
        eb.subscribe(Box::new(ErrorHandler)).await.unwrap();

        let rx = eb.publish_all(vec![create_event("ok")]).await.unwrap();
        assert_eq!(rx.await.unwrap(), 1);

        let rx = eb.publish_all(vec![create_event("error")]).await.unwrap();
        assert_eq!(rx.await.unwrap(), 0);
    }
}
