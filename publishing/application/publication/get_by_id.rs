use serde::Serialize;

use common::event::EventPublisher;
use common::request::Include;
use common::result::Result;

use crate::application::dtos::{AuthorDto, CategoryDto, PublicationDto, ReaderInteractionDto};
use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::content_manager::{ContentManagerId, ContentManagerRepository};
use crate::domain::interaction::InteractionService;
use crate::domain::publication::{PublicationId, PublicationRepository, StatisticsService};
use crate::domain::reader::{ReaderId, ReaderRepository};

#[derive(Serialize)]
pub struct GetByIdResponse {
    pub publication: PublicationDto,
    pub reader: Option<ReaderInteractionDto>,
}

pub struct GetById<'a> {
    event_pub: &'a dyn EventPublisher,

    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    content_manager_repo: &'a dyn ContentManagerRepository,
    publication_repo: &'a dyn PublicationRepository,
    reader_repo: &'a dyn ReaderRepository,

    interaction_serv: &'a InteractionService,
    statistics_serv: &'a StatisticsService,
}

impl<'a> GetById<'a> {
    pub fn new(
        event_pub: &'a dyn EventPublisher,
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        content_manager_repo: &'a dyn ContentManagerRepository,
        publication_repo: &'a dyn PublicationRepository,
        reader_repo: &'a dyn ReaderRepository,
        interaction_serv: &'a InteractionService,
        statistics_serv: &'a StatisticsService,
    ) -> Self {
        GetById {
            event_pub,
            author_repo,
            category_repo,
            content_manager_repo,
            publication_repo,
            reader_repo,
            interaction_serv,
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
            self.content_manager_repo
                .find_by_id(&ContentManagerId::new(auth_id)?)
                .await
                .is_ok()
        } else {
            false
        };

        let (mut publication_dto, reader_interaction_dto) = if let Some(auth_id) = auth_id {
            let is_reader_author = publication.author_id().value() == auth_id;

            if is_reader_author {
                (
                    PublicationDto::from(&publication)
                        .status(&publication)
                        .pages(&publication),
                    None,
                )
            } else if is_content_manager {
                (
                    PublicationDto::from(&publication).status(&publication),
                    None,
                )
            } else {
                let reader_id = ReaderId::new(auth_id)?;
                let reader = self.reader_repo.find_by_id(&reader_id).await?;

                self.interaction_serv
                    .add_view(&reader, &mut publication)
                    .await?;

                self.publication_repo.save(&mut publication).await?;

                self.event_pub
                    .publish_all(publication.base().events()?)
                    .await?;

                let reader_statistics = self
                    .statistics_serv
                    .get_history(Some(&reader_id), Some(&publication_id), None, None)
                    .await?;

                (
                    PublicationDto::from(&publication),
                    Some(ReaderInteractionDto::new(
                        reader_statistics.views() > 0,
                        reader_statistics.readings() > 0,
                        reader_statistics.likes() > 0,
                        reader_statistics.reviews() > 0,
                    )),
                )
            }
        } else {
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

    use crate::mocks;

    #[tokio::test]
    async fn owner_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.content_manager_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let mut reader = mocks::author_as_reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                Some(reader.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add("author").add("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, author.base().id().value());
        assert_eq!(res.name, publication.header().name().value());
        assert_eq!(
            res.category.unwrap().id,
            publication.header().category_id().value()
        );
        assert!(res.pages.unwrap().len() > 0);
        assert_eq!(res.statistics.views, 0);
        assert_eq!(res.statistics.unique_views, 0);
        assert_eq!(res.statistics.readings, 0);
        assert_eq!(res.status.unwrap(), "draft");

        assert_eq!(c.event_pub().events().await.len(), 0);
    }

    #[tokio::test]
    async fn reader_view_of_draft() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.content_manager_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                Some(reader.base().id().to_string()),
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
            c.content_manager_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        let res = uc
            .exec(
                Some(reader.base().id().to_string()),
                publication.base().id().to_string(),
                Include::default().add("author").add("category"),
            )
            .await
            .unwrap();
        let res = res.publication;
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author.unwrap().id, publication.author_id().value());
        assert!(res.pages.is_none());
        assert_eq!(res.statistics.views, 1);
        assert_eq!(res.statistics.unique_views, 1);
        assert!(res.status.is_none());

        assert!(c.event_pub().events().await.len() > 0);
    }

    #[tokio::test]
    async fn invalid_id() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.author_repo(),
            c.category_repo(),
            c.content_manager_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        assert!(uc
            .exec(
                Some(reader.base().id().to_string()),
                "#invalid".to_owned(),
                Include::default()
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
            c.content_manager_repo(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
            c.statistics_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut author = mocks::author1();
        c.author_repo().save(&mut author).await.unwrap();
        let mut category = mocks::category1();
        c.category_repo().save(&mut category).await.unwrap();

        c.interaction_serv()
            .add_like(&reader, &mut publication)
            .await
            .unwrap();

        let res = uc
            .exec(
                Some(reader.base().id().to_string()),
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
