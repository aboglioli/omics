use serde::Serialize;

use common::event::EventPublisher;
use common::result::Result;

use crate::domain::publication::{Publication, PublicationId, PublicationRepository};
use crate::domain::reader::{ReaderId, ReaderRepository};
use crate::domain::statistics::{Statistics, StatisticsRepository};

#[derive(Serialize)]
pub struct StatisticsDto {
    views: u32,
    unique_views: u32,
    readings: u32,
    likes: u32,
    reviews: u32,
    stars: f32,
}

impl From<&Statistics> for StatisticsDto {
    fn from(statistics: &Statistics) -> Self {
        StatisticsDto {
            views: statistics.views(),
            unique_views: statistics.unique_views(),
            readings: statistics.readings(),
            likes: statistics.likes(),
            reviews: statistics.reviews(),
            stars: statistics.stars(),
        }
    }
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
pub struct ViewResponse {
    id: String,
    author_id: String,
    name: String,
    synopsis: String,
    category_id: String,
    tags: Vec<String>,
    pages: Vec<PageDto>,
    status: Option<String>,
    statistics: Option<StatisticsDto>,
}

impl From<&Publication> for ViewResponse {
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

        ViewResponse {
            id: publication.base().id().value().to_owned(),
            author_id: publication.author_id().value().to_owned(),
            name: header.name().value().to_owned(),
            synopsis: header.synopsis().value().to_owned(),
            category_id: header.category_id().value().to_owned(),
            tags,
            pages,
            status: None,
            statistics: None,
        }
    }
}

pub struct View<'a, EPub, PRepo, RRepo, SRepo> {
    event_pub: &'a EPub,

    publication_repo: &'a PRepo,
    reader_repo: &'a RRepo,
    statistics_repo: &'a SRepo,
}

impl<'a, EPub, PRepo, RRepo, SRepo> View<'a, EPub, PRepo, RRepo, SRepo>
where
    EPub: EventPublisher,
    PRepo: PublicationRepository,
    RRepo: ReaderRepository,
    SRepo: StatisticsRepository,
{
    pub fn new(
        event_pub: &'a EPub,
        publication_repo: &'a PRepo,
        reader_repo: &'a RRepo,
        statistics_repo: &'a SRepo,
    ) -> Self {
        View {
            event_pub,
            publication_repo,
            reader_repo,
            statistics_repo,
        }
    }

    pub async fn exec(&self, reader_id: String, publication_id: String) -> Result<ViewResponse> {
        let publication_id = PublicationId::new(publication_id)?;
        let mut publication = self.publication_repo.find_by_id(&publication_id).await?;

        let reader_id = ReaderId::new(reader_id)?;
        let reader = self.reader_repo.find_by_id(&reader_id).await?;

        let mut res = ViewResponse::from(&publication);

        if publication.author_id() == &reader_id {
            res.status = Some(publication.status_history().current().status().to_string());
        } else {
            publication.view(&reader)?;

            self.event_pub
                .publish_all(publication.base().events()?)
                .await?;
        }

        let statistics = match self
            .statistics_repo
            .find_by_publication_id(&publication.base().id())
            .await
        {
            Ok(statistics) => statistics,
            Err(_) => Statistics::default(publication.base().id()),
        };

        res.statistics = Some(StatisticsDto::from(&statistics));

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
        let uc = View::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.statistics_repo(),
        );

        let mut reader = mocks::author_as_reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut statistics =
            Statistics::new(publication.base().id(), 32, 32, 32, 32, 32, 4.8).unwrap();
        c.statistics_repo().save(&mut statistics).await.unwrap();
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
        assert_eq!(res.status.unwrap(), "draft");

        assert_eq!(c.event_pub().events().await.len(), 0);
    }

    #[tokio::test]
    async fn reader_view_of_draft() {
        let c = mocks::container();
        let uc = View::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.statistics_repo(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut statistics =
            Statistics::new(publication.base().id(), 32, 32, 32, 32, 32, 4.8).unwrap();
        c.statistics_repo().save(&mut statistics).await.unwrap();

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
        let uc = View::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.statistics_repo(),
        );

        let mut reader = mocks::reader1();
        c.reader_repo().save(&mut reader).await.unwrap();
        let mut publication = mocks::published_publication1();
        c.publication_repo().save(&mut publication).await.unwrap();
        let mut statistics =
            Statistics::new(publication.base().id(), 32, 32, 32, 32, 32, 4.8).unwrap();
        c.statistics_repo().save(&mut statistics).await.unwrap();

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
        assert!(res.status.is_none());

        assert_eq!(c.event_pub().events().await.len(), 1);
    }

    #[tokio::test]
    async fn default_statistics() {
        let c = mocks::container();
        let uc = View::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.statistics_repo(),
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

        let statistics = res.statistics.unwrap();
        assert_eq!(statistics.views, 0);
        assert_eq!(statistics.unique_views, 0);
        assert_eq!(statistics.readings, 0);
        assert_eq!(statistics.likes, 0);
        assert_eq!(statistics.reviews, 0);
        assert_eq!(statistics.stars, 0.0);
    }

    #[tokio::test]
    async fn invalid_id() {
        let c = mocks::container();
        let uc = View::new(
            c.event_pub(),
            c.publication_repo(),
            c.reader_repo(),
            c.statistics_repo(),
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
