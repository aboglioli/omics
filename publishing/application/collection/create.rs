use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;
use identity::UserIdAndRole;
use common::error::Error;

use crate::domain::author::{AuthorId, AuthorRepository};
use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::collection::{Collection, CollectionRepository};
use crate::domain::publication::{Header, Image, Name, Synopsis, Tag};

#[derive(Deserialize)]
pub struct CreateCommand {
    pub name: String,
    pub synopsis: String,
    pub category_id: String,
    pub tags: Vec<String>,
    pub cover: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    id: String,
}

pub struct Create<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> Create<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        Create {
            event_pub,
            author_repo,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(&self, (auth_id, auth_role): UserIdAndRole, cmd: CreateCommand) -> Result<CreateResponse> {
        if !auth_role.can("create_collection") {
            return Err(Error::unauthorized());
        }

        let name = Name::new(cmd.name)?;
        let synopsis = Synopsis::new(cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.into_iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(cmd.cover)?;

        let category_id = CategoryId::new(cmd.category_id)?;
        self.category_repo.find_by_id(&category_id).await?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        self.author_repo.find_by_id(&auth_id).await?;

        let mut collection =
            Collection::new(self.collection_repo.next_id().await?, auth_id, header)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.events().to_vec()?)
            .await?;

        Ok(CreateResponse {
            id: collection.base().id().to_string(),
        })
    }
}
