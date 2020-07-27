use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use glob::Pattern;

use crate::error::Error;
use crate::event::{Event, EventPublisher, EventSubscriber, EventWithTopic, Subscription};

struct SubscriptionWithTopic<'a> {
    topic: String,
    subscription: Subscription<'a>,
}

impl<'a> SubscriptionWithTopic<'a> {
    fn new(topic: &str, subscription: Subscription<'a>) -> Self {
        SubscriptionWithTopic {
            topic: topic.to_owned(),
            subscription,
        }
    }

    fn topic(&self) -> &str {
        &self.topic
    }

    fn subscription(&mut self) -> &mut Subscription<'a> {
        &mut self.subscription
    }
}

pub struct InMemEventBus<'a> {
    subscriptions: RefCell<Vec<SubscriptionWithTopic<'a>>>,
}

impl InMemEventBus<'_> {
    pub fn new() -> Self {
        Self {
            subscriptions: RefCell::new(Vec::new()),
        }
    }
}

impl Default for InMemEventBus<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPublisher for InMemEventBus<'_> {
    type Output = usize;

    fn publish(&self, topic: &str, event: &dyn Event) -> Result<Self::Output, Error> {
        let mut count = 0;
        let mut errs = Error::internal();

        for sub_item in self.subscriptions.borrow_mut().iter_mut() {
            let pattern = Pattern::new(sub_item.topic()).unwrap();
            if pattern.matches(topic) {
                if let Err(err) = (sub_item.subscription())(topic, event) {
                    errs.merge(err);
                }
                count += 1;
            }
        }

        if errs.has_context() {
            return Err(errs);
        }

        Ok(count)
    }

    fn publish_all(&self, events_with_topic: &[EventWithTopic]) -> Result<Self::Output, Error> {
        let mut count = 0;
        let mut errs = Error::internal();

        for ewt in events_with_topic.iter() {
            let count = match self.publish(ewt.topic(), ewt.event()) {
                Ok(c) => count + c,
                Err(err) => {
                    errs.merge(err);
                    0
                }
            };
        }

        if errs.has_context() {
            return Err(errs);
        }

        Ok(count)
    }
}

impl<'a> EventSubscriber<'a> for InMemEventBus<'a> {
    type Output = bool;

    fn subscribe(&self, topic: &str, cb: Subscription<'a>) -> Result<Self::Output, Error> {
        if let Ok(mut subscriptions) = self.subscriptions.try_borrow_mut() {
            subscriptions.push(SubscriptionWithTopic::new(topic, cb));
            return Ok(true);
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct BasicEvent {
        code: String,
    }

    impl BasicEvent {
        fn new(code: &str) -> Self {
            BasicEvent {
                code: code.to_owned(),
            }
        }
    }

    impl Event for BasicEvent {
        fn code(&self) -> &str {
            &self.code
        }
        fn payload(&self) -> Vec<u8> {
            self.code.as_bytes().to_vec()
        }
    }

    #[test]
    fn create() {
        let eb = InMemEventBus::new();
        assert_eq!(eb.subscriptions.borrow().len(), 0);
    }

    #[test]
    fn polymorphic() {
        let eb = InMemEventBus::new();

        let emitter: &dyn EventPublisher<Output = _> = &eb;
        let subscriber: &dyn EventSubscriber<Output = _> = &eb;

        subscriber
            .subscribe("topic", Box::new(|_, _| Ok(())))
            .unwrap();
        subscriber
            .subscribe("topic", Box::new(|_, _| Ok(())))
            .unwrap();
        subscriber
            .subscribe("topic", Box::new(|_, _| Ok(())))
            .unwrap();
        assert_eq!(
            emitter
                .publish("topic", &BasicEvent::new("evented"))
                .unwrap(),
            3
        );
    }

    #[test]
    fn publish_subscribe() {
        let (mut called1, mut called2, mut called3, mut called4) = (false, false, false, false);
        {
            let eb = InMemEventBus::new();
            assert!(eb
                .subscribe(
                    "ent1.created",
                    Box::new(|topic, event| {
                        called1 = true;
                        assert_eq!(topic, "ent1.created");
                        assert_eq!(event.code(), "ent1.created");
                        Ok(())
                    }),
                )
                .unwrap(),);
            assert!(eb
                .subscribe(
                    "ent1.created",
                    Box::new(|topic, event| {
                        called2 = true;
                        assert_eq!(topic, "ent1.created");
                        assert_eq!(event.code(), "ent1.created");
                        Ok(())
                    }),
                )
                .unwrap(),);
            assert!(eb
                .subscribe(
                    "ent1.updated",
                    Box::new(|topic, event| {
                        called3 = true;
                        assert_eq!(topic, "ent1.updated");
                        assert_eq!(event.code(), "ent1.updated");
                        Ok(())
                    }),
                )
                .unwrap(),);
            assert!(eb
                .subscribe(
                    "ent2.created",
                    Box::new(|topic, event| {
                        called4 = true;
                        assert_eq!(topic, "ent2.created");
                        assert_eq!(event.code(), "ent2.created");
                        Ok(())
                    }),
                )
                .unwrap(),);
            assert_eq!(
                eb.publish("ent1.created", &BasicEvent::new("ent1.created"))
                    .unwrap(),
                2
            );
            assert_eq!(
                eb.publish("ent2.created", &BasicEvent::new("ent2.created"))
                    .unwrap(),
                1
            );
        }
        assert!(called1);
        assert!(called2);
        assert!(!called3);
        assert!(called4);
    }

    #[test]
    fn glob_topics() {
        let (mut calls1, mut calls2, mut calls3, mut calls4, mut calls5, mut calls6) =
            (0, 0, 0, 0, 0, 0);
        {
            let eb = InMemEventBus::new();
            eb.subscribe(
                "ent.created",
                Box::new(|_, _| {
                    calls1 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.updated",
                Box::new(|_, _| {
                    calls2 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.deleted",
                Box::new(|_, _| {
                    calls3 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.*",
                Box::new(|_, _| {
                    calls4 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "*.created",
                Box::new(|_, _| {
                    calls5 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "*.*",
                Box::new(|_, _| {
                    calls6 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            assert_eq!(
                eb.publish("ent.created", &BasicEvent::new("ent.created"))
                    .unwrap(),
                4
            );
            assert_eq!(
                eb.publish("ent.updated", &BasicEvent::new("ent.updated"))
                    .unwrap(),
                3
            );
            assert_eq!(
                eb.publish("another.created", &BasicEvent::new("another.created"))
                    .unwrap(),
                2
            );
        }
        assert_eq!(calls1, 1);
        assert_eq!(calls2, 1);
        assert_eq!(calls3, 0);
        assert_eq!(calls4, 2);
        assert_eq!(calls5, 2);
        assert_eq!(calls6, 3);
    }

    #[test]
    fn publish_all() {
        let mut count = 0;
        let mut topics = Vec::new();
        let mut codes = Vec::new();
        {
            let eb = InMemEventBus::new();
            let events_with_topic = vec![
                EventWithTopic::new("evt1.created", BasicEvent::new("code1")),
                EventWithTopic::new("evt2.created", BasicEvent::new("code2")),
                EventWithTopic::new("evt1.updated", BasicEvent::new("code3")),
            ];

            eb.subscribe(
                "*.*",
                Box::new(|topic, event| {
                    topics.push(topic.to_owned());
                    codes.push(event.code().to_owned());
                    count += 1;
                    Ok(())
                }),
            );

            eb.publish_all(&events_with_topic);
        }
        assert_eq!(count, 3);
        assert_eq!(topics[0], "evt1.created");
        assert_eq!(topics[1], "evt2.created");
        assert_eq!(topics[2], "evt1.updated");
        assert_eq!(codes[0], "code1");
        assert_eq!(codes[1], "code2");
        assert_eq!(codes[2], "code3");
    }
}
