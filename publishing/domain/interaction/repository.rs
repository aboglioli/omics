use async_trait::async_trait;
use chrono::{DateTime, Utc};

use common::result::Result;

use crate::domain::interaction::{Interaction, Kind};
use crate::domain::publication::PublicationId;
use crate::domain::reader::ReaderId;

pub struct FindOpts<'a> {
    pub reader_id: Option<&'a ReaderId>,
    pub publication_id: Option<&'a PublicationId>,
    pub kind: Option<&'a Kind>,
    pub from: Option<&'a DateTime<Utc>>,
    pub to: Option<&'a DateTime<Utc>>,
}

#[async_trait]
pub trait InteractionRepository {
    async fn find(&self, opts: &FindOpts<'_>) -> Result<Vec<Interaction>>;

    async fn save(&self, interaction: &mut Interaction) -> Result<()>;
}
