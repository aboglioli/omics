use crate::error::Error;
use crate::event::EventHandler;

pub trait EventSubscriber {
    type Output;

    fn subscribe(
        &self,
        handler: Box<dyn EventHandler<Output = Self::Output>>,
    ) -> Result<Self::Output, Error>;
}
