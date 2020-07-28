use crate::error::Error;
use crate::event::Event;

pub trait EventHandler: Send {
    type Output;

    fn topic(&self) -> &str;

    fn handle(&mut self, event: &Event) -> Result<Self::Output, Error>;
}
