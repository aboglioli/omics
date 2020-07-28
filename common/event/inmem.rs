use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use regex::Regex;

use crate::error::Error;
use crate::event::{Event, EventHandler, EventPublisher, EventSubscriber};

pub struct InMemEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<Output = bool>>>>>,
    emitter: Option<Sender<Event>>,
    pending_events: Arc<AtomicU32>,
}

impl InMemEventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
            emitter: None,
            pending_events: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn pending_events(&self) -> u32 {
        self.pending_events.load(Ordering::Relaxed)
    }

    pub fn run(&mut self) {
        let (emitter, rx) = mpsc::channel();

        self.emitter = Some(emitter);

        let handlers = Arc::clone(&self.handlers);
        let pending_events = Arc::clone(&self.pending_events);

        thread::spawn(move || {
            for event in rx.iter() {
                if event.code() == "__quit" {
                    pending_events.store(0, Ordering::Relaxed);
                    return;
                }

                for handler in handlers.lock().unwrap().iter_mut() {
                    if let Ok(re) = Regex::new(handler.topic()) {
                        if re.is_match(event.topic()) {
                            if let Err(err) = handler.handle(&event) {
                                println!("{}", err);
                            }
                        }
                    }
                }

                pending_events.fetch_sub(1, Ordering::Relaxed);
            }
        });
    }

    pub fn stop(&self) -> Result<bool, Error> {
        self.publish(Event::new("", "__quit", Vec::new()))
    }
}

impl Default for InMemEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPublisher for InMemEventBus {
    type Output = bool;

    fn publish(&self, event: Event) -> Result<Self::Output, Error> {
        if let Some(emitter) = &self.emitter {
            if let Err(err) = emitter.send(event) {
                return Err(Error::internal().wrap_raw(err).build());
            }
            self.pending_events.fetch_add(1, Ordering::Relaxed);
            return Ok(true);
        }
        Err(Error::internal()
            .set_code("event_bus")
            .set_message("not_initialized")
            .build())
    }

    fn publish_all(&self, events: Vec<Event>) -> Result<Self::Output, Error> {
        for event in events.into_iter() {
            self.publish(event)?;
        }

        Ok(true)
    }
}

impl EventSubscriber for InMemEventBus {
    type Output = bool;

    fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output>>,
    ) -> Result<Self::Output, Error> {
        let mut handlers = self.handlers.lock().unwrap();
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

    impl EventHandler for BasicHandler {
        type Output = bool;

        fn topic(&self) -> &str {
            &self.topic
        }

        fn handle(&mut self, event: &Event) -> Result<Self::Output, Error> {
            self.counter.inc(event.topic());
            Ok(true)
        }
    }

    #[test]
    fn create() {
        let eb = InMemEventBus::new();
        assert_eq!(eb.handlers.lock().unwrap().len(), 0);
    }

    #[test]
    fn polymorphic() {
        let mut eb = InMemEventBus::new();
        assert!(eb.publish(create_event("evented")).is_err());

        eb.run();

        let publisher: &dyn EventPublisher<Output = _> = &eb;
        let subscriber: &dyn EventSubscriber<Output = _> = &eb;
        let _counter = Arc::new(Counter::new());
        let handler = BasicHandler::new(".*");

        subscriber.subscribe(Box::new(handler.clone())).unwrap();
        subscriber.subscribe(Box::new(handler.clone())).unwrap();
        subscriber.subscribe(Box::new(handler.clone())).unwrap();
        subscriber.subscribe(Box::new(handler.clone())).unwrap();

        assert!(publisher.publish(create_event("evented")).is_ok());
        assert!(eb.stop().is_ok());

        loop {
            if eb.pending_events() == 0 {
                break;
            }
        }

        assert_eq!(handler.counter().count("evented"), 4);

        let _: Box<dyn EventPublisher<Output = _>> = Box::new(InMemEventBus::new());
    }

    #[test]
    fn publish_subscribe() {
        let mut eb = InMemEventBus::new();
        eb.run();

        let handler = BasicHandler::new("*");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.created")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.created")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent1.updated")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent2.created")))
            .is_ok());

        assert!(eb.publish(create_event("ent1.created")).is_ok());
        assert!(eb.publish(create_event("ent2.created")).is_ok());

        assert!(eb.stop().is_ok());

        loop {
            if eb.pending_events() == 0 {
                break;
            }
        }

        assert_eq!(handler.counter().count("ent1.created"), 2);
        assert_eq!(handler.counter().count("ent1.updated"), 0);
        assert_eq!(handler.counter().count("ent2.created"), 1);
    }

    #[test]
    fn match_topics_with_regex() {
        let mut eb = InMemEventBus::new();
        eb.run();

        let handler = BasicHandler::new(".+");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(".+")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(".+.created")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.+")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.created")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.updated")))
            .is_ok());
        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic("ent.(deleted|updated)")))
            .is_ok());

        assert!(eb.publish(create_event("ent.created")).is_ok());
        assert!(eb.publish(create_event("ent.updated")).is_ok());
        assert!(eb.publish(create_event("ent.deleted")).is_ok());

        assert!(eb.stop().is_ok());

        loop {
            if eb.pending_events() == 0 {
                break;
            }
        }

        assert_eq!(handler.counter().count("ent.created"), 4);
        assert_eq!(handler.counter().count("ent.updated"), 4);
        assert_eq!(handler.counter().count("ent.deleted"), 3);
    }

    #[test]
    fn publish_all() {
        let mut eb = InMemEventBus::new();
        eb.run();

        let handler = BasicHandler::new(".+");

        assert!(eb
            .subscribe(Box::new(handler.clone_with_topic(r"ent.+|.+.created")))
            .is_ok());

        assert!(eb
            .publish_all(vec![
                create_event("another.created"),
                create_event("ent.created"),
                create_event("ent.updated"),
                create_event("ent.deleted"),
                create_event("another.updated"),
            ])
            .is_ok());

        assert!(eb.stop().is_ok());

        loop {
            if eb.pending_events() == 0 {
                break;
            }
        }

        assert_eq!(handler.counter().count("ent.created"), 1);
        assert_eq!(handler.counter().count("ent.updated"), 1);
        assert_eq!(handler.counter().count("ent.deleted"), 1);
        assert_eq!(handler.counter().count("another.created"), 1);
        assert_eq!(handler.counter().count("another.updated"), 0);
    }
}
