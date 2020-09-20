use common::event::EventPublisher;
use common::result::Result;

use crate::container::IdentityContainer;

pub async fn populate<EPub>(_c: &IdentityContainer<EPub>) -> Result<()>
where
    EPub: EventPublisher,
{
    Ok(())
}
