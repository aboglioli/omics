use common::event::EventPublisher;
use common::result::Result;

use crate::container::Container;

pub async fn populate<EPub>(_c: &Container<EPub>) -> Result<()>
where
    EPub: EventPublisher,
{
    Ok(())
}
