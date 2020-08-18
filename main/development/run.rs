use common::event::EventSubscriber;
use common::result::Result;

use crate::container::Container;
use crate::development::EventLogger;

pub async fn run(c: &Container) -> Result<()> {
    let event_bus = c.event_bus();
    event_bus.subscribe(Box::new(EventLogger::new())).await?;

    Ok(())
}
