use async_trait::async_trait;

use common::result::Result;

use crate::domain::publication::{Publication, PublicationId};

#[async_trait]
pub trait PublicationRepository {
<<<<<<< HEAD:publishing/domain/publication/repository.rs
    async fn next_id(&self) -> Result<PublicationId>;

=======
>>>>>>> backend:payment/domain/publication/repository.rs
    async fn find_by_id(&self, id: &PublicationId) -> Result<Publication>;

    async fn save(&self, publication: &mut Publication) -> Result<()>;
}
