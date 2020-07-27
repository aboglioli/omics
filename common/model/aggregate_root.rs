use std::clone::Clone;
use std::cmp::PartialEq;

use chrono::{DateTime, Utc};

use crate::event::EventWithTopic;

#[derive(Debug)]
pub struct AggregateRoot<ID> {
    id: ID,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
    events: Vec<EventWithTopic>,
}

impl<ID: Clone> AggregateRoot<ID> {
    pub fn new(id: ID) -> AggregateRoot<ID> {
        AggregateRoot {
            id,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            events: Vec::new(),
        }
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    pub fn update(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn delete(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn record_event(&mut self, event: EventWithTopic) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[EventWithTopic] {
        &self.events
    }

    pub fn clean_events(&mut self) {
        self.events.clear();
    }
}

impl<ID: PartialEq> PartialEq for AggregateRoot<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ID: Clone> Clone for AggregateRoot<ID> {
    fn clone(&self) -> Self {
        AggregateRoot {
            id: self.id.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
            events: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type AggRootID = String;
    struct AggRoot {
        base: AggregateRoot<AggRootID>,
        name: String,
    }

    impl AggRoot {
        fn new(id: AggRootID, name: &str) -> AggRoot {
            AggRoot {
                base: AggregateRoot::new(id),
                name: name.to_owned(),
            }
        }

        fn base(&self) -> &AggregateRoot<AggRootID> {
            &self.base
        }

        fn base_mut(&mut self) -> &mut AggregateRoot<AggRootID> {
            &mut self.base
        }
    }

    #[test]
    fn create() {
        let e = AggRoot::new(AggRootID::from("AR_022"), "I'm an aggregate root");
        assert_eq!(e.base().id(), "AR_022");
    }

    #[test]
    fn properties() {
        let mut e = AggRoot::new(AggRootID::from("AR_022"), "I'm an aggregate root");
        assert_eq!(e.base().id(), "AR_022");
        assert_ne!(e.base().created_at(), &Utc::now());
        assert!(e.base().created_at() < &Utc::now());
        assert!(e.base().updated_at().is_none());
        assert!(e.base().deleted_at().is_none());

        e.base_mut().update();
        assert!(e.base().updated_at().is_some());
        assert_ne!(e.base().updated_at().unwrap(), &Utc::now());

        e.base_mut().delete();
        assert!(e.base().deleted_at().is_some());
        assert_ne!(e.base().deleted_at().unwrap(), &Utc::now());
    }

    #[test]
    fn equals() {
        let mut ag1 = AggRoot::new(AggRootID::from("AR_101"), "Agg 1");
        let mut ag2 = AggRoot::new(AggRootID::from("AR_101"), "Agg 2");

        assert_eq!(ag1.base(), ag2.base());
    }
}
