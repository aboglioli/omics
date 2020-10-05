use serde::Serialize;

use common::result::Result;

use crate::domain::author::AuthorRepository;
use crate::domain::category::CategoryRepository;
use crate::domain::collection::CollectionRepository;
use crate::domain::publication::PublicationRepository;

use crate::application::dtos::PublicationDto;

#[derive(Default, Serialize)]
pub struct Catalog {
    new_publications: Vec<PublicationDto>,
    most_viewed_publications: Vec<PublicationDto>,
    most_liked_publications: Vec<PublicationDto>,
    best_review_publications: Vec<PublicationDto>,
}

pub struct GetCatalog<'a> {
    author_repo: &'a dyn AuthorRepository,
    category_repo: &'a dyn CategoryRepository,
    collection_repo: &'a dyn CollectionRepository,
    publication_repo: &'a dyn PublicationRepository,
}

impl<'a> GetCatalog<'a> {
    pub fn new(
        author_repo: &'a dyn AuthorRepository,
        category_repo: &'a dyn CategoryRepository,
        collection_repo: &'a dyn CollectionRepository,
        publication_repo: &'a dyn PublicationRepository,
    ) -> Self {
        GetCatalog {
            author_repo,
            category_repo,
            collection_repo,
            publication_repo,
        }
    }

    pub async fn exec(&self) -> Result<Catalog> {
        let mut publications = self
            .publication_repo
            .search(None, None, Some(&"published".to_owned()), None)
            .await?;

        let mut catalog = Catalog::default();

        publications.sort_by(|a, b| a.base().created_at().cmp(b.base().created_at()));
        for publication in publications.iter() {
            catalog
                .new_publications
                .push(PublicationDto::from(publication));
        }

        publications.sort_by(|a, b| a.statistics().views().cmp(&b.statistics().views()));
        publications.reverse();
        for publication in publications.iter() {
            catalog
                .most_viewed_publications
                .push(PublicationDto::from(publication));
        }

        publications.sort_by(|a, b| a.statistics().likes().cmp(&b.statistics().likes()));
        publications.reverse();
        for publication in publications.iter() {
            catalog
                .most_liked_publications
                .push(PublicationDto::from(publication));
        }

        publications.sort_by(|a, b| {
            a.statistics()
                .stars()
                .partial_cmp(&b.statistics().stars())
                .unwrap()
        });
        publications.reverse();
        for publication in publications.iter() {
            catalog
                .best_review_publications
                .push(PublicationDto::from(publication));
        }

        Ok(catalog)
    }
}
