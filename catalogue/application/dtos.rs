use serde::Serialize;

use crate::domain::catalogue::{Author, Catalogue, Category, Publication, Statistics};

#[derive(Serialize)]
pub struct StatisticsDto {
    pub views: u32,
    pub unique_views: u32,
    pub readings: u32,
    pub likes: u32,
    pub reviews: u32,
    pub stars: f32,
}

impl StatisticsDto {
    pub fn new(statistics: &Statistics) -> Self {
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
pub struct AuthorDto {
    pub id: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
}

impl AuthorDto {
    pub fn new(author: &Author) -> Self {
        AuthorDto {
            id: author.id().to_string(),
            username: author.username().to_string(),
            name: author.name().to_string(),
            lastname: author.lastname().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
}

impl CategoryDto {
    pub fn new(category: &Category) -> Self {
        CategoryDto {
            id: category.id().to_string(),
            name: category.name().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicationDto {
    pub id: String,
    pub author: AuthorDto,
    pub name: String,
    pub synopsis: String,
    pub category: CategoryDto,
    pub tags: Vec<String>,
    pub cover: String,
    pub statistics: StatisticsDto,
    pub premium: bool,
    pub pages: usize,
}

impl PublicationDto {
    pub fn new(publication: &Publication) -> Self {
        PublicationDto {
            id: publication.id().to_string(),
            author: AuthorDto::new(publication.author()),
            name: publication.name().to_string(),
            synopsis: publication.synopsis().to_string(),
            category: CategoryDto::new(publication.category()),
            tags: publication
                .tags()
                .iter()
                .map(|tag| tag.to_string())
                .collect(),
            cover: publication.cover().to_string(),
            statistics: StatisticsDto::new(publication.statistics()),
            premium: publication.is_premium(),
            pages: publication.pages(),
        }
    }
}

#[derive(Serialize)]
pub struct CatalogueDto {
    id: String,
    authors: Vec<AuthorDto>,
    publications: Vec<PublicationDto>,
}

impl CatalogueDto {
    pub fn new(catalogue: &Catalogue) -> Self {
        CatalogueDto {
            id: catalogue.base().id().to_string(),
            authors: catalogue
                .authors()
                .iter()
                .map(|author| AuthorDto::new(author))
                .collect(),
            publications: catalogue
                .publications()
                .iter()
                .map(|publication| PublicationDto::new(publication))
                .collect(),
        }
    }
}
