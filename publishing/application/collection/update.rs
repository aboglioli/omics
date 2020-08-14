use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::result::Result;

use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::{Header, Image, Name, Synopsis, Tag};

#[derive(Deserialize)]
pub struct UpdateCommand {
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    cover: String,
}

impl UpdateCommand {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

pub struct Update<'a, EPub, CatRepo, CollRepo> {
    event_pub: &'a EPub,

    category_repo: &'a CatRepo,
    collection_repo: &'a CollRepo,
}

impl<'a, EPub, CatRepo, CollRepo> Update<'a, EPub, CatRepo, CollRepo>
where
    EPub: EventPublisher,
    CatRepo: CategoryRepository,
    CollRepo: CollectionRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        category_repo: &'a CatRepo,
        collection_repo: &'a CollRepo,
    ) -> Self {
        Update {
            event_pub,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        author_id: String,
        collection_id: String,
        cmd: UpdateCommand,
    ) -> Result<()> {
        cmd.validate()?;

        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        if collection.author_id().value() != author_id {
            return Err(Error::new("collection", "unauthorized"));
        }

        let name = Name::new(cmd.name)?;
        let synopsis = Synopsis::new(cmd.synopsis)?;

        let mut tags = Vec::new();
        for tag in cmd.tags.iter() {
            tags.push(Tag::new(tag)?);
        }

        let cover = Image::new(cmd.cover)?;

        let category_id = CategoryId::new(cmd.category_id)?;
        self.category_repo.find_by_id(&category_id).await?;

        let header = Header::new(name, synopsis, category_id, tags, cover)?;

        collection.set_header(header)?;

        self.collection_repo.save(&mut collection).await?;

        self.event_pub
            .publish_all(collection.base().events()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[tokio::test]
    async fn update() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let author = mocks::author1();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        uc.exec(
            author.base().id().value().to_owned(),
            collection.base().id().value().to_owned(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().value().to_owned(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
            },
        )
        .await
        .unwrap();

        let collection = c
            .collection_repo()
            .find_by_id(&collection.base().id())
            .await
            .unwrap();
        assert_eq!(collection.header().name().value(), "New name");
        assert_eq!(collection.header().synopsis().value(), "New synopsis...");
        assert_eq!(collection.header().category_id().value(), "#category02");

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn published_publication() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let author = mocks::author1();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        uc.exec(
            author.base().id().value().to_owned(),
            collection.base().id().value().to_owned(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().value().to_owned(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
            },
        )
        .await
        .unwrap();

        let _collection = c
            .collection_repo()
            .find_by_id(&collection.base().id())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let author = mocks::author2();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();
        let mut category = mocks::category2();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                author.base().id().value().to_owned(),
                collection.base().id().value().to_owned(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().value().to_owned(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn non_existing_category() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let author = mocks::author1();
        let mut collection = mocks::empty_collection1();
        c.collection_repo().save(&mut collection).await.unwrap();
        let category = mocks::category2();

        assert!(uc
            .exec(
                author.base().id().value().to_owned(),
                collection.base().id().value().to_owned(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().value().to_owned(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }
}
