use std::cell::RefCell;

use crate::common::error::Error;
use crate::common::event::event::{Event, EventPublisher};

pub struct InMemEventPublisher {
    events: RefCell<Vec<Box<dyn Event>>>,
}

impl InMemEventPublisher {
    pub fn new() -> InMemEventPublisher {
        InMemEventPublisher {
            events: RefCell::new(Vec::new()),
        }
    }
}

impl EventPublisher for InMemEventPublisher {
    fn publish<E: Event + 'static>(&self, _: &str, event: E) -> Result<(), Error> {
        self.events.borrow_mut().push(Box::new(event));
        Ok(())
    }
}
