use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use glob::glob;

use crate::error::Error;
use crate::event::{Event, EventPublisher, EventSubscriber, Subscription};

pub struct InMemEventBus<'a> {
    subscriptions: RefCell<HashMap<String, Vec<Subscription<'a>>>>,
}

impl InMemEventBus<'_> {
    pub fn new() -> Self {
        Self {
            subscriptions: RefCell::new(HashMap::new()),
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

        if let Some(subs) = self.subscriptions.borrow_mut().get_mut(topic) {
            for sub in subs.iter_mut() {
                if let Err(err) = sub(event) {
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
}

impl<'a> EventSubscriber<'a> for InMemEventBus<'a> {
    type Output = usize;

    fn subscribe(&self, topic: &str, cb: Subscription<'a>) -> Result<Self::Output, Error> {
        let mut subscribed = 0;

        if let Ok(mut subscriptions) = self.subscriptions.try_borrow_mut() {
            if let Some(subs) = subscriptions.get_mut(topic) {
                subs.push(cb);
                subscribed = subs.len();
            } else {
                subscriptions.insert(topic.to_owned(), vec![cb]);
                subscribed = 1;
            }
        }

        Ok(subscribed)
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

        subscriber.subscribe("topic", Box::new(|_| Ok(()))).unwrap();
        subscriber.subscribe("topic", Box::new(|_| Ok(()))).unwrap();
        subscriber.subscribe("topic", Box::new(|_| Ok(()))).unwrap();
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
            assert_eq!(
                eb.subscribe(
                    "ent1.created",
                    Box::new(|event| {
                        called1 = true;
                        assert_eq!(event.code(), "ent1.created");
                        Ok(())
                    }),
                )
                .unwrap(),
                1
            );
            assert_eq!(
                eb.subscribe(
                    "ent1.created",
                    Box::new(|event| {
                        called2 = true;
                        assert_eq!(event.code(), "ent1.created");
                        Ok(())
                    }),
                )
                .unwrap(),
                2
            );
            assert_eq!(
                eb.subscribe(
                    "ent1.updated",
                    Box::new(|event| {
                        called3 = true;
                        assert_eq!(event.code(), "ent1.updated");
                        Ok(())
                    }),
                )
                .unwrap(),
                1
            );
            assert_eq!(
                eb.subscribe(
                    "ent2.created",
                    Box::new(|event| {
                        called4 = true;
                        assert_eq!(event.code(), "ent2.created");
                        Ok(())
                    }),
                )
                .unwrap(),
                1
            );
            assert_eq!(eb.publish("ent1.created", &BasicEvent::new("ent1.created")).unwrap(), 2);
            assert_eq!(eb.publish("ent2.created", &BasicEvent::new("ent2.created")).unwrap(), 1);
        }
        assert!(called1);
        assert!(called2);
        assert!(!called3);
        assert!(called4);
    }

    #[test]
    fn glob() {
        let (mut calls1, mut calls2, mut calls3, mut calls4, mut calls5, mut calls6) = (0, 0, 0, 0, 0, 0);
        {
            let eb = InMemEventBus::new();
            eb.subscribe(
                "ent.created",
                Box::new(|event| {
                    calls1 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.updated",
                Box::new(|event| {
                    calls2 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.deleted",
                Box::new(|event| {
                    calls3 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "ent.*",
                Box::new(|event| {
                    calls4 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "*.created",
                Box::new(|event| {
                    calls5 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            eb.subscribe(
                "*.*",
                Box::new(|event| {
                    calls6 += 1;
                    Ok(())
                }),
            )
            .unwrap();
            assert_eq!(eb.publish("ent.created", &BasicEvent::new("ent.created")).unwrap(), 4);
            assert_eq!(eb.publish("ent.updated", &BasicEvent::new("ent.updated")).unwrap(), 3);
            assert_eq!(eb.publish("another.created", &BasicEvent::new("another.created")).unwrap(), 2);
        }
        assert_eq!(calls1, 1);
        assert_eq!(calls2, 1);
        assert_eq!(calls3, 0);
        assert_eq!(calls4, 2);
        assert_eq!(calls5, 2);
        assert_eq!(calls6, 3);
    }
}
