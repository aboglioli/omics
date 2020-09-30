use crate::event::{Event, ToEvent};
use crate::result::Result;

#[derive(Debug, Default)]
pub struct Events<E> {
    events: Vec<E>,
}

impl<E> Events<E>
where
    E: ToEvent,
{
    pub fn new() -> Self {
        Events { events: Vec::new() }
    }

    pub fn record_event(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn to_vec(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for event in self.events.iter() {
            events.push(event.to_event()?);
        }
        Ok(events)
    }
}

impl<E> Clone for Events<E>
where
    E: ToEvent,
{
    fn clone(&self) -> Self {
        Self::new()
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

    #[derive(Debug)]
    pub struct AggRoot {
        events: Events<AggRootEvent>,
    }

    impl AggRoot {
        fn new() -> Self {
            AggRoot {
                events: Events::new(),
            }
        }

        fn events(&self) -> &Events<AggRootEvent> {
            &self.events
        }

        fn exec(&mut self) {
            self.events.record_event(AggRootEvent::Created {
                text: "agg_root.created".to_owned(),
            });
            self.events.record_event(AggRootEvent::Updated { num: 123 });
            self.events.record_event(AggRootEvent::Deleted(true));
        }
    }

    #[test]
    fn events() {
        let mut ag = AggRoot::new();
        ag.exec();

        let events = ag.events().to_vec().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].topic(), "agg_root.created");
        assert_eq!(events[0].payload(), "agg_root.created".as_bytes());
        assert_eq!(events[1].topic(), "agg_root.updated");
        assert_eq!(events[1].payload(), vec![255]);
        assert_eq!(events[2].topic(), "agg_root.deleted");
        assert_eq!(events[2].payload(), vec![1]);
    }
}
