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

    pub fn has(&self, event: &dyn Event) -> bool {
        for evt in self.events.borrow().iter() {
            if evt.code() == event.code() && evt.payload() == event.payload() {
                return true;
            }
        }
        false
    }
}

impl EventPublisher for InMemEventPublisher {
    fn publish(&self, _: &str, event: Box<dyn Event>) -> Result<(), Error> {
        self.events.borrow_mut().push(event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct EntityCreated;
    impl Event for EntityCreated {
        fn code(&self) -> &str {
            "entity-created"
        }
        fn payload(&self) -> Vec<u8> {
            b"entity-created".to_vec()
        }
    }

    #[derive(Debug)]
    struct EntityUpdated;
    impl Event for EntityUpdated {
        fn code(&self) -> &str {
            "entity-updated"
        }
        fn payload(&self) -> Vec<u8> {
            b"entity-updated".to_vec()
        }
    }

    #[derive(Debug)]
    struct AnotherEvent;
    impl Event for AnotherEvent {
        fn code(&self) -> &str {
            "another"
        }
        fn payload(&self) -> Vec<u8> {
            b"another".to_vec()
        }
    }

    fn publish() -> Result<(), Error> {
        let event_publisher = InMemEventPublisher::new();
        event_publisher.publish("entity.created", Box::new(EntityCreated))?;
        event_publisher.publish("entity.updated", Box::new(EntityUpdated))?;
        assert_eq!(event_publisher.events().borrow().len(), 2);
        assert!(event_publisher.has(&EntityCreated));
        assert!(event_publisher.has(&EntityUpdated));
        assert!(!event_publisher.has(&AnotherEvent));

        let events = event_publisher.events().borrow();
        assert_eq!(events.get(0).unwrap().code(), "entitiy-created");
        assert_eq!(events.get(1).unwrap().code(), "entitiy-updated");

        Ok(())
    }
}
