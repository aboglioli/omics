use serde::Deserialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::CommandResponse;
use common::result::Result;
use identity::UserIdAndRole;

use crate::domain::category::{CategoryId, CategoryRepository};
use crate::domain::collection::{CollectionId, CollectionRepository};
use crate::domain::publication::{Header, Image, Name, Synopsis, Tag};

#[derive(Deserialize)]
pub struct UpdateCommand {
    pub name: String,
    pub synopsis: String,
    pub category_id: String,
    pub tags: Vec<String>,
    pub cover: String,
}

pub struct Update<'a> {
    event_pub: &'a dyn EventPublisher,

    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
}

impl<'a> Update<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
    ) -> Self {
        Update {
            event_pub,
            category_repo,
            collection_repo,
        }
    }

    pub async fn exec(
        &self,
        (auth_id, auth_role): UserIdAndRole,
        collection_id: String,
        cmd: UpdateCommand,
    ) -> Result<CommandResponse> {
        if !auth_role.can("update_collection") {
            return Err(Error::unauthorized());
        }

        let collection_id = CollectionId::new(collection_id)?;
        let mut collection = self.collection_repo.find_by_id(&collection_id).await?;

        if collection.author_id() != &auth_id {
            return Err(Error::not_owner("collection"));
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
            .publish_all(collection.events().to_vec()?)
            .await?;

        Ok(CommandResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity::domain::user::UserId;
    use identity::mocks as identity_mocks;

    use crate::mocks;

    #[tokio::test]
    async fn update() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let mut collection = mocks::collection(
            "#collection01",
            "#user01",
            "Name",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "cover.jpg",
        );
        c.collection_repo().save(&mut collection).await.unwrap();
        let mut category = mocks::category("Category 2");
        c.category_repo().save(&mut category).await.unwrap();
        let role = identity_mocks::role("User");

        uc.exec(
            (UserId::new("#user01").unwrap(), role),
            collection.base().id().to_string(),
            UpdateCommand {
                name: "New name".to_owned(),
                synopsis: "New synopsis...".to_owned(),
                category_id: category.base().id().to_string(),
                tags: vec!["New tag".to_owned()],
                cover: "domain.com/new-cover.jpg".to_owned(),
            },
        )
        .await
        .unwrap();

        let collection = c
            .collection_repo()
            .find_by_id(collection.base().id())
            .await
            .unwrap();
        assert_eq!(collection.header().name().value(), "New name");
        assert_eq!(collection.header().synopsis().value(), "New synopsis...");
        assert_eq!(collection.header().category_id().value(), "category-2");

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn not_owner() {
        let c = mocks::container();
        let uc = Update::new(c.event_pub(), c.category_repo(), c.collection_repo());

        let mut collection = mocks::collection(
            "#collection01",
            "#user01",
            "User",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "cover.jpg",
        );
        c.collection_repo().save(&mut collection).await.unwrap();
        let mut category = mocks::category("Category 2");
        c.category_repo().save(&mut category).await.unwrap();
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (UserId::new("#user02").unwrap(), role),
                collection.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
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

        let mut collection = mocks::collection(
            "#collection01",
            "#user01",
            "User",
            "category-1",
            vec!["Tag 1", "Tag 2"],
            "cover.jpg",
        );
        c.collection_repo().save(&mut collection).await.unwrap();
        let category = mocks::category("Category 2");
        let role = identity_mocks::role("User");

        assert!(uc
            .exec(
                (UserId::new("#user01").unwrap(), role),
                collection.base().id().to_string(),
                UpdateCommand {
                    name: "New name".to_owned(),
                    synopsis: "New synopsis...".to_owned(),
                    category_id: category.base().id().to_string(),
                    tags: vec!["New tag".to_owned()],
                    cover: "domain.com/new-cover.jpg".to_owned(),
                },
            )
            .await
            .is_err());
    }
}
