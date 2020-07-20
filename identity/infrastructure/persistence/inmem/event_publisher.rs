use std::cell::RefCell;

use common::error::Error;
use common::event::{Event, EventPublisher};

pub struct InMemEventPublisher {
    events: RefCell<Vec<Box<dyn Event>>>,
}

impl InMemEventPublisher {
    pub fn new() -> InMemEventPublisher {
        InMemEventPublisher {
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn events(&self) -> &RefCell<Vec<Box<dyn Event>>> {
        &self.events
    }

    pub fn has<E: Event + 'static>(&self, event: E) -> bool {
        for evt in self.events.borrow().iter() {
            if evt.code() == event.code() && evt.payload() == event.payload() {
                return true;
            }
        }
        false
    }
}

impl EventPublisher for InMemEventPublisher {
    fn publish<E: Event + 'static>(&self, _: &str, event: E) -> Result<(), Error> {
        self.events.borrow_mut().push(Box::new(event));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct EntityCreated;
    #[derive(Debug)]
    struct EntityUpdated;

    impl Event for EntityCreated {
        fn code(&self) -> &str {
            "entity-created"
        }
        fn payload(&self) -> Vec<u8> {
            b"entity-created".to_vec()
        }
    }

    impl Event for EntityUpdated {
        fn code(&self) -> &str {
            "entity-updated"
        }
        fn payload(&self) -> Vec<u8> {
            b"entity-updated".to_vec()
        }
    }

    fn publish() -> Result<(), Error> {
        let event_publisher = InMemEventPublisher::new();
        event_publisher.publish("entity.created", EntityCreated)?;
        event_publisher.publish("entity.updated", EntityUpdated)?;
        assert_eq!(event_publisher.events().borrow().len(), 2);
        assert!(event_publisher.has(EntityCreated));
        assert!(event_publisher.has(EntityUpdated));

        let events = event_publisher.events();
        assert_eq!(events.borrow().get(0).unwrap().code(), "entitiy-created");
        assert_eq!(events.borrow().get(1).unwrap().code(), "entitiy-updated");

        Ok(())
    }
}
