use serde::Serialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::interaction::{InteractionRepository, InteractionService};
use crate::domain::publication::{Publication, PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};

#[derive(Serialize)]
pub struct StatisticsDto {
    views: u32,
    unique_views: u32,
    readings: u32,
    likes: u32,
    reviews: u32,
    stars: f32,
}

#[derive(Serialize)]
pub struct ImageDto {
    url: String,
}

#[derive(Serialize)]
pub struct PageDto {
    number: u32,
    images: Vec<ImageDto>,
}

#[derive(Serialize)]
pub struct GetByIdResponse {
    id: String,
    author_id: String,
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    pages: Vec<PageDto>,
    statistics: StatisticsDto,
    status: Option<String>,
}

impl From<&Publication> for GetByIdResponse {
    fn from(publication: &Publication) -> Self {
        let mut pages = Vec::new();
        for page in publication.pages().iter() {
            let mut images = Vec::new();
            for image in page.images().iter() {
                images.push(ImageDto {
                    url: image.url().to_owned(),
                });
            }

            pages.push(PageDto {
                number: *page.number(),
                images,
            });
        }

        let header = publication.header();

        let tags: Vec<String> = header.tags().iter().map(|t| t.name().to_owned()).collect();

        let statistics = publication.statistics();
        let statistics = StatisticsDto {
            views: statistics.views(),
            unique_views: statistics.unique_views(),
            readings: statistics.readings(),
            likes: statistics.likes(),
            reviews: statistics.reviews(),
            stars: statistics.stars(),
        };

        GetByIdResponse {
            id: publication.base().id().value().to_owned(),
            author_id: publication.author_id().value().to_owned(),
            name: header.name().value().to_owned(),
            synopsis: header.synopsis().value().to_owned(),
            category_id: header.category_id().value().to_owned(),
            tags,
            pages,
            status: None,
            statistics,
        }
    }
}

pub struct GetById<'a, EPub, PRepo, RRepo, IRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,

    interaction_serv: InteractionService<'a, IRepo>,
}

impl<'a, EPub, PRepo, RRepo, IRepo> GetById<'a, EPub, PRepo, RRepo, IRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    IRepo: InteractionRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
        interaction_serv: InteractionService<'a, IRepo>,
    ) -> Self {
        GetById {
            event_pub,
            publication_repo,
            reader_repo,
            interaction_serv,
        }
    }

    pub async fn exec(&self, reader_id: String, publication_id: String) -> Result<GetByIdResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(reader_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        let is_reader_author = publication.author_id() == &reader_id;

        if !is_reader_author {
            self.interaction_serv
                .add_view(&reader, &mut publication)
                .await?;

            self.publication_repo.save(&mut publication).await?;

            self.event_pub
                .publish_all(publication.base().events()?)
                .await?;
        }

        let mut res = GetByIdResponse::from(&publication);
        if is_reader_author {
            res.status = Some(publication.status_history().current().status().to_string());
        }

        Ok(res)
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
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::author_as_reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let author = mocks::author1();

        let res = uc
            .exec(
                reader.base().id().value().to_owned(),
                publication.base().id().value().to_owned(),
            )
            .await
            .unwrap();
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author_id, author.base().id().value());
        assert_eq!(res.name, publication.header().name().value());
        assert_eq!(res.category_id, publication.header().category_id().value());
        assert_eq!(res.pages.len(), 0);
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
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        assert!(uc
            .exec(
                reader.base().id().value().to_owned(),
                publication.base().id().value().to_owned(),
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn reader_view_of_published() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();

        let res = uc
            .exec(
                reader.base().id().value().to_owned(),
                publication.base().id().value().to_owned(),
            )
            .await
            .unwrap();
        assert_eq!(res.id, publication.base().id().value());
        assert_eq!(res.author_id, publication.author_id().value());
        assert_eq!(res.pages.len(), 2);
        assert_eq!(res.statistics.views, 1);
        assert_eq!(res.statistics.unique_views, 1);
        assert!(res.status.is_none());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn invalid_id() {
        let c = mocks::container();
        let uc = GetById::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.interaction_serv(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let _publication = mocks::published_publication1();

        assert!(uc
            .exec(reader.base().id().value().to_owned(), "#invalid".to_owned(),)
            .await
            .is_err());
    }
}
