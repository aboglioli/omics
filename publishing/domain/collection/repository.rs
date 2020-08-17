use async_trait::async_trait;

use common::result::Result;

use crate::domain::author::AuthorId;
use crate::domain::category::CategoryId;
use crate::domain::collection::{Collection, CollectionId};

#[async_trait]
pub trait CollectionRepository {
    async fn next_id(&self) -> Result<CollectionId>;

    async fn find_by_id(&self, id: &CollectionId) -> Result<Collection>;
    async fn find_by_author_id(&self, author_id: &AuthorId) -> Result<Vec<Collection>>;
    async fn find_by_category_id(&self, category_id: &CategoryId) -> Result<Vec<Collection>>;
    async fn search(&self, text: &str) -> Result<Vec<Collection>>;

    async fn save(&self, collection: &mut Collection) -> Result<()>;
}
