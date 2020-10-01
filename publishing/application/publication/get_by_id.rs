use serde::Serialize;

use common::error::Error;
use common::event::EventPublisher;
use common::request::Include;
use common::result::Result;
use identity::domain::user::{UserId, UserRepository};

use crate::application::dtos::{
    AuthorDto, CategoryDto, PublicationDto, ReaderPublicationInteractionDto,
};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::interaction::InteractionRepository;
use crate::domain::publication::{PublicationId, PublicationRepository, StatisticsService};
use crate::domain::reader::{ReaderId, ReaderRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    pub publication: PublicationDto,
    pub reader: Option<ReaderPublicationInteractionDto>,
}

pub struct GetById<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    interaction_repo: &'a dyn InteractionRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,
    user_repo: &'a dyn UserRepository,

    statistics_serv: &'a StatisticsService,
}

impl<'a> GetById<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        interaction_repo: &'a dyn InteractionRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        user_repo: &'a dyn UserRepository,
        statistics_serv: &'a StatisticsService,
    ) -> Self {
        GetById {
            event_pub,
            author_repo,
            category_repo,
            interaction_repo,
            publication_repo,
            reader_repo,
            user_repo,
            statistics_serv,
        }
    }

    pub async fn exec(
        &self,
        auth_id: Option<String>,
        publication_id: String,
        include: Include,
    ) -> Result<GetByIdResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let is_content_manager = if let Some(auth_id) = &auth_id {
            let user = self.user_repo.find_by_id(&UserId::new(auth_id)?).await?;
            user.is_content_manager()
        } else {
            false
        };

        let (mut publication_dto, reader_interaction_dto) = if let Some(auth_id) = auth_id {
            let is_reader_author = publication.author_id().value() == auth_id;

            if is_reader_author {
                (PublicationDto::from(&publication).pages(&publication), None)
            } else if is_content_manager {
                (PublicationDto::from(&publication), None)
            } else {
                let reader_id = ReaderId::new(auth_id)?;
                let reader = self.reader_repo.find_by_id(&reader_id).await?;

                let mut view = publication.view(
                    &reader,
                    self.interaction_repo
                        .find_views(Some(&reader_id), Some(&publication_id), None, None)
                        .await?
                        .is_empty(),
                )?;

                self.interaction_repo.save_view(&mut view).await?;
                self.publication_repo.save(&mut publication).await?;

                self.event_pub
                    .publish_all(publication.events().to_vec()?)
                    .await?;

                let reader_statistics = self
                    .statistics_serv
                    .get_history(Some(&reader_id), Some(&publication_id), None, None)
                    .await?;

                let in_favorites = !self
                    .interaction_repo
                    .find_publication_favorites(Some(&reader_id), Some(&publication_id), None, None)
                    .await?
                    .is_empty();

                (
                    PublicationDto::from(&publication),
                    Some(ReaderPublicationInteractionDto::new(
                        reader_statistics.views() > 0,
                        reader_statistics.readings() > 0,
                        reader_statistics.likes() > 0,
                        reader_statistics.reviews() > 0,
                        in_favorites,
                    )),
                )
            }
        } else {
            if !publication.is_published() {
                return Err(Error::new("publication", "not_published"));
            }

            (PublicationDto::from(&publication), None)
        };

        if include.has("author") {
            let author = self.author_repo.find_by_id(publication.author_id()).await?;
            publication_dto = publication_dto.author(AuthorDto::from(&author));
        }

        if include.has("category") {
            let category = self
                .category_repo
                .find_by_id(publication.header().category_id())
                .await?;
            publication_dto = publication_dto.category(CategoryDto::from(&category));
        }

        Ok(GetByIdResponse {
            publication: publication_dto,
            reader: reader_interaction_dto,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity::domain::user::User;
    use identity::mocks as identity_mocks;

    use crate::domain::author::Author;
    use crate::domain::reader::Reader;
    use crate::mocks;

    fn user(index: u32) -> (User, Author, Reader) {
        (
            identity_mocks::user(
                &format!("#user0{}", index),
                &format!("user-{}", index),
                &format!("user-{}@omics.com", index),
                "P@asswd!",
                true,
                Some("Name"),
                Some("Lastname"),
                "user",
            ),
            mocks::author(&format!("#user0{}", index), &format!("user-{}", index)),
            mocks::reader(&format!("#user0{}", index)),
        )
    }

    #[tokio::test]
    async fn owner_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = user(1);
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = user(2);
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category("#category01", "Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        let res = uc
            .exec(
                Some(reader1.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add_field("author").add_field("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, author1.base().id().value());
        assert_eq!(res.name, publication.header().name().value());
        assert_eq!(
            res.category.unwrap().id,
            publication.header().category_id().value()
        );
        assert!(res.pages.unwrap().len() > 0);
        assert_eq!(res.statistics.views, 0);
        assert_eq!(res.statistics.unique_views, 0);
        assert_eq!(res.statistics.readings, 0);
        assert_eq!(res.status.status, "draft");
        assert!(res.status.changed_by.is_none());

        assert_eq!(c.event_pub().events().await.len(), 0);
    }

    #[tokio::test]
    async fn reader_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = user(1);
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = user(2);
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category("#category01", "Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            false,
            false,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default(),
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn reader_view_of_published() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = user(1);
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = user(2);
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category("#category01", "Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        let res = uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add_field("author").add_field("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, publication.author_id().value());
        assert!(res.pages.is_none());
        assert_eq!(res.statistics.views, 1);
        assert_eq!(res.statistics.unique_views, 1);
        assert_eq!(res.status.status, "published");
        assert!(res.status.changed_by.is_some());

        assert!(c.event_pub().events().await.len() > 0);
    }

    #[tokio::test]
    async fn invalid_id() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = user(1);
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = user(2);
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category("#category01", "Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                Some(reader1.base().id().to_string()),
                "#invalid".to_owned(),
                Include::default(),
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn reader_interaction() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.interaction_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.user_repo(),
            c.statistics_serv(),
        );

        let (mut user1, mut author1, mut reader1) = user(1);
        c.user_repo().save(&mut user1).await.unwrap();
        c.author_repo().save(&mut author1).await.unwrap();
        c.reader_repo().save(&mut reader1).await.unwrap();

        let (mut user2, mut author2, mut reader2) = user(2);
        c.user_repo().save(&mut user2).await.unwrap();
        c.author_repo().save(&mut author2).await.unwrap();
        c.reader_repo().save(&mut reader2).await.unwrap();

        let mut category = mocks::category("#category01", "Category 1");
        c.category_repo().save(&mut category).await.unwrap();

        let mut publication = mocks::publication(
            "#publication01",
            "#user01",
            "Publication 01",
            "#category01",
            vec!["Tag 1", "Tag 2"],
            "domain.com/cover.jpg",
            3,
            true,
            true,
        );
        c.publication_repo().save(&mut publication).await.unwrap();

        let mut like = publication.like(&reader2).unwrap();
        c.interaction_repo().save_like(&mut like).await.unwrap();

        let res = uc
            .exec(
                Some(reader2.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default(),
            )
            .await
            .unwrap();
        let res = res.reader.unwrap();
        assert!(res.viewed);
        assert!(res.liked);
        assert!(!res.read);
        assert!(!res.reviewed);
    }
}
