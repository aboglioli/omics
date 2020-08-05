use std::clone::Clone;
use std::cmp::PartialEq;

use chrono::{DateTime, Utc};

use crate::error::Error;
use crate::event::{Event, ToEvent};
use crate::result::Result;

#[derive(Debug)]
pub struct BasicEvent;

impl ToEvent for BasicEvent {
    fn to_event(&self) -> Result<Event> {
        Err(Error::internal("to_event", "not_implemented"))
    }
}

#[derive(Debug)]
pub struct AggregateRoot<ID, E> {
    id: ID,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
    events: Vec<E>,
}

impl<ID: Clone, E: ToEvent> AggregateRoot<ID, E> {
    pub fn new(id: ID) -> AggregateRoot<ID, E> {
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

    pub fn record_event(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn events(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for event in self.events.iter() {
            events.push(event.to_event()?);
        }
        Ok(events)
    }
}

impl<ID: PartialEq, E> PartialEq for AggregateRoot<ID, E> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ID: Clone, E> Clone for AggregateRoot<ID, E> {
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

    #[derive(Debug)]
    enum AggRootEvent {
        Created { text: String },
        Updated { num: u32 },
        Deleted(bool),
    }

    impl ToEvent for AggRootEvent {
        fn to_event(&self) -> Result<Event> {
            Ok(match self {
                AggRootEvent::Created { text } => {
                    Event::new("agg_root.created", "", text.as_bytes().to_vec())
                }
                AggRootEvent::Updated { num } => Event::new(
                    "agg_root.updated",
                    "",
                    vec![if num < &255 { 255 } else { 0 }],
                ),
                AggRootEvent::Deleted(_v) => Event::new("agg_root.deleted", "", vec![1]),
            })
        }
    }

    type AggRootID = String;

    #[derive(Debug)]
    struct AggRoot {
        base: AggregateRoot<AggRootID, AggRootEvent>,
    }

    impl AggRoot {
        fn new(id: AggRootID) -> AggRoot {
            AggRoot {
                base: AggregateRoot::new(id),
            }
        }

        fn base(&self) -> &AggregateRoot<AggRootID, AggRootEvent> {
            &self.base
        }

        fn base_mut(&mut self) -> &mut AggregateRoot<AggRootID, AggRootEvent> {
            &mut self.base
        }
    }

    #[test]
    fn create() {
        let e = AggRoot::new(AggRootID::from("AR_022"));
        assert_eq!(e.base().id(), "AR_022");
    }

    #[test]
    fn properties() {
        let mut e = AggRoot::new(AggRootID::from("AR_022"));
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
        let ag1 = AggRoot::new(AggRootID::from("AR_101"));
        let ag2 = AggRoot::new(AggRootID::from("AR_101"));

        assert_eq!(ag1.base(), ag2.base());
    }

    #[test]
    fn events() {
        let mut ag = AggRoot::new(AggRootID::from("AR_08"));
        ag.base_mut().record_event(AggRootEvent::Created {
            text: "agg_root.created".to_owned(),
        });
        ag.base_mut()
            .record_event(AggRootEvent::Updated { num: 32 });
        ag.base_mut().record_event(AggRootEvent::Deleted(true));

        let events = ag.base().events().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].topic(), "agg_root.created");
        assert_eq!(events[0].payload(), &"agg_root.created".as_bytes().to_vec());
        assert_eq!(events[1].topic(), "agg_root.updated");
        assert_eq!(events[2].topic(), "agg_root.deleted");
    }
}
