use std::sync::Arc;

use async_trait::async_trait;
use regex::Regex;
use tokio::sync::oneshot::{self, Receiver};
use tokio::sync::Mutex;

use crate::error::Error;
use crate::event::{Event, EventHandler, EventPublisher, EventSubscriber, PublicationResult};
use crate::result::Result;

#[derive(Default)]
pub struct InMemEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler + Sync + Send>>>>,
}

impl InMemEventBus {
    pub fn new() -> Self {
        InMemEventBus {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl EventPublisher for InMemEventBus {
    async fn publish(&self, event: Event) -> Result<Receiver<PublicationResult>> {
        self.publish_all(vec![event]).await
    }

    async fn publish_all(&self, events: Vec<Event>) -> Result<Receiver<PublicationResult>> {
        let handlers = Arc::clone(&self.handlers);
        let (tx, rx) = oneshot::channel();
        let mut publication_result = PublicationResult::default();

        tokio::spawn(async move {
            for event in events.into_iter() {
                let mut handlers = handlers.lock().await;
                for handler in handlers.iter_mut() {
                    match Regex::new(handler.topic()) {
                        Ok(re) => {
                            if re.is_match(event.topic()) {
                                // Execute handler
                                if let Err(err) = handler.handle(&event).await {
                                    let err = Error::internal("event_publisher", "handler_error")
                                        .wrap(err)
                                        .build();
                                    println!("{:?}", err);

                                    publication_result.err_handlers += 1;
                                } else {
                                    publication_result.ok_handlers += 1;
                                }
                            }
                        }
                        Err(err) => {
                            let err = Error::internal("event_publisher", "invalid_topic_regex")
                                .wrap_raw(err)
                                .build();
                            println!("{:?}", err);
                        }
                    }
                }

                publication_result.published_events += 1;
            }

            if tx.send(publication_result).is_err() {}
        });

        Ok(rx)
    }
}

#[async_trait]
impl EventSubscriber for InMemEventBus {
    async fn subscribe(&self, handler: Box<dyn EventHandler + Sync + Send>) -> Result<bool> {
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
        fn new<S: Into<String>>(topic: S) -> Self {
            BasicHandler {
                counter: Arc::new(Counter::new()),
                topic: topic.into(),
            }
        }

        pub fn counter(&self) -> &Counter {
            &self.counter
        }

        pub fn clone_with_topic<S: Into<String>>(&self, topic: S) -> Self {
            BasicHandler {
                counter: Arc::clone(&self.counter),
                topic: topic.into(),
            }
        }

        pub fn clone(&self) -> Self {
            self.clone_with_topic(&self.topic)
        }
    }

    #[async_trait]
    impl EventHandler for BasicHandler {
        fn topic(&self) -> &str {
            &self.topic
        }

        async fn handle(&mut self, event: &Event) -> Result<bool> {
            self.counter.inc(event.topic());
            Ok(true)
        }
    }

    struct ErrorHandler;

    #[async_trait]
    impl EventHandler for ErrorHandler {
        fn topic(&self) -> &str {
            "error.*"
        }

        async fn handle(&mut self, _: &Event) -> Result<bool> {
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
        let _: Box<dyn EventSubscriber> = Box::new(InMemEventBus::new());
        let _: Box<dyn EventPublisher> = Box::new(InMemEventBus::new());

        let mut eb = InMemEventBus::new();
        let subscriber: &mut dyn EventSubscriber = &mut eb;
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

        let publisher: &mut dyn EventPublisher = &mut eb;
        let res = publisher
            .publish(create_event("evented"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.activated_handlers(), 4);
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

        let res = eb
            .publish(create_event("ent1.created"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.ok_handlers(), 2);
        assert_eq!(res.err_handlers(), 0);
        assert_eq!(res.activated_handlers(), 2);

        let res = eb
            .publish(create_event("ent2.created"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.ok_handlers(), 1);
        assert_eq!(res.err_handlers(), 0);
        assert_eq!(res.activated_handlers(), 1);

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

        let res = eb
            .publish(create_event("ent.created"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.activated_handlers(), 4);

        let res = eb
            .publish(create_event("ent.updated"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.activated_handlers(), 4);

        let res = eb
            .publish(create_event("ent.deleted"))
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.activated_handlers(), 3);

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

        let res = eb
            .publish_all(vec![
                create_event("another.created"),
                create_event("ent.created"),
                create_event("ent.updated"),
                create_event("ent.deleted"),
                create_event("another.updated"),
            ])
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 5);
        assert_eq!(res.ok_handlers(), 4);
        assert_eq!(res.err_handlers(), 0);
        assert_eq!(res.activated_handlers(), 4);

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

        let res = eb
            .publish_all(vec![create_event("ok")])
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.ok_handlers(), 2);
        assert_eq!(res.err_handlers(), 0);

        let res = eb
            .publish_all(vec![create_event("error")])
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 1);
        assert_eq!(res.ok_handlers(), 1);
        assert_eq!(res.err_handlers(), 1);

        let res = eb
            .publish_all(vec![create_event("ok"), create_event("error")])
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(res.published_events(), 2);
        assert_eq!(res.ok_handlers(), 3);
        assert_eq!(res.err_handlers(), 1);
    }
}
