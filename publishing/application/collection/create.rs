use serde::{Deserialize, Serialize};

use common::event::EventPublisher;
use common::result::Result;

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

impl CreateCommand {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CreateResponse {
    id: String,
}

pub struct Create<'a, EPub, ARepo, CatRepo, CollRepo> {
    event_pub: &'a EPub,

    author_repo: &'a ARepo,
    category_repo: &'a CatRepo,
    collection_repo: &'a CollRepo,
}

impl<'a, EPub, ARepo, CatRepo, CollRepo> Create<'a, EPub, ARepo, CatRepo, CollRepo>
where
    EPub: EventPublisher,
    ARepo: AuthorRepository,
    CatRepo: CategoryRepository,
    CollRepo: CollectionRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        author_repo: &'a ARepo,
        category_repo: &'a CatRepo,
        collection_repo: &'a CollRepo,
    ) -> Self {
        Create {
            event_pub,
            author_repo,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(&self, author_id: String, cmd: CreateCommand) -> Result<CreateResponse> {
        cmd.validate()?;

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

        let author_id = AuthorId::new(author_id)?;
        self.author_repo.find_by_id(&author_id).await?;

        let mut collection =
            Collection::new(self.collection_repo.next_id().await?, author_id, header)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(CreateResponse {
            id: collection.base().id().value().to_owned(),
        })
    }
}
