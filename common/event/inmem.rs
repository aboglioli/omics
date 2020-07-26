use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::error::Error;
use crate::event::{Event, EventPublisher, EventSubscriber, Subscription};

pub struct InMemEventBus {
    subscriptions: RefCell<HashMap<String, Vec<Subscription>>>,
    notified: Cell<u32>,
}

impl InMemEventBus {
    pub fn new() -> InMemEventBus {
        InMemEventBus {
            subscriptions: RefCell::new(HashMap::new()),
            notified: Cell::new(0),
        }
    }

    pub fn notified(&self) -> u32 {
        self.notified.get()
    }

    pub fn reset_counter(&self) {
        self.notified.set(0);
    }
}

impl EventPublisher for InMemEventBus {
    fn publish(&self, topic: &str, event: Box<dyn Event>) -> Result<(), Error> {
        let mut count = 0;

        if let Some(subs) = self.subscriptions.borrow_mut().get_mut(topic) {
            for sub in subs.iter_mut() {
                sub(event.as_ref());
                count += 1;
            }
        }

        self.notified.set(self.notified.get() + count);

        Ok(())
    }
}

impl EventSubscriber for InMemEventBus {
    fn subscribe(&self, topic: &str, cb: Subscription) -> Result<(), Error> {
        if let Some(mut subs) = self.subscriptions.borrow_mut().get_mut(topic) {
            subs.push(cb);
            return Ok(());
        }

        self.subscriptions
            .borrow_mut()
            .insert(topic.to_owned(), vec![cb]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn create() {
        let eb = InMemEventBus::new();
        assert_eq!(eb.subscriptions.borrow().len(), 0);
    }

    #[derive(Debug)]
    struct Ent1Created;
    impl Event for Ent1Created {
        fn code(&self) -> &str {
            "ent1.created"
        }
        fn payload(&self) -> Vec<u8> {
            b"ent1.created".to_vec()
        }
    }

    #[derive(Debug)]
    struct Ent1Updated;
    impl Event for Ent1Updated {
        fn code(&self) -> &str {
            "ent1.updated"
        }
        fn payload(&self) -> Vec<u8> {
            b"ent1.updated".to_vec()
        }
    }

    #[derive(Debug)]
    struct Ent2Created;
    impl Event for Ent2Created {
        fn code(&self) -> &str {
            "ent2.created"
        }
        fn payload(&self) -> Vec<u8> {
            b"ent2.created".to_vec()
        }
    }

    #[test]
    fn publish_subscribe() {
        let eb = InMemEventBus::new();
        let calls = Rc::new(Cell::new(0));

        let call = Rc::clone(&calls);
        eb.subscribe(
            "ent1.created",
            Box::new(move |event| {
                call.set(call.get() + 1);
                assert_eq!(event.code(), "ent1.created");
                Ok(())
            }),
        );

        let call = Rc::clone(&calls);
        eb.subscribe(
            "ent1.created",
            Box::new(move |event| {
                call.set(call.get() + 1);
                Ok(())
            }),
        );

        let call = Rc::clone(&calls);
        eb.subscribe(
            "ent1.updated",
            Box::new(move |event| {
                call.set(call.get() + 1);
                assert_eq!(event.code(), "ent1.updated");
                Ok(())
            }),
        );

        let call = Rc::clone(&calls);
        eb.subscribe(
            "ent2.created",
            Box::new(move |event| {
                call.set(call.get() + 1);
                assert_eq!(event.code(), "ent2.created");
                Ok(())
            }),
        );

        eb.publish("ent1.created", Box::new(Ent1Created)).unwrap();
        assert_eq!(calls.get(), 2);
        assert_eq!(eb.notified(), 2);

        eb.reset_counter();
        calls.set(0);
        eb.publish("ent1.created", Box::new(Ent1Created)).unwrap();
        eb.publish("ent1.updated", Box::new(Ent1Updated)).unwrap();
        eb.publish("ent2.created", Box::new(Ent2Created)).unwrap();
        assert_eq!(calls.get(), 4);
        assert_eq!(eb.notified(), 4);
    }
}
