mod repository;
pub use repository::*;

use common::error::Error;
use common::result::Result;

use crate::domain::publication::PublicationId;

#[derive(Debug, Clone)]
pub struct Statistics {
    publication_id: PublicationId,
    views: u32,
    unique_views: u32,
    readings: u32,
    likes: u32,
    reviews: u32,
    stars: f32,
}

impl Statistics {
    pub fn new(
        publication_id: PublicationId,
        views: u32,
        unique_views: u32,
        readings: u32,
        likes: u32,
        reviews: u32,
        stars: f32,
    ) -> Result<Statistics> {
        if stars < 0.0 {
            return Err(Error::new("statistics", "stars_not_positive"));
        }

        Ok(Statistics {
            publication_id,
            views,
            unique_views,
            readings,
            likes,
            reviews,
            stars,
        })
    }

    pub fn default(publication_id: PublicationId) -> Self {
        Self::new(publication_id, 0, 0, 0, 0, 0, 0.0).unwrap()
    }

    pub fn publication_id(&self) -> &PublicationId {
        &self.publication_id
    }

    pub fn views(&self) -> u32 {
        self.views
    }

    pub fn unique_views(&self) -> u32 {
        self.unique_views
    }

    pub fn readings(&self) -> u32 {
        self.readings
    }

    pub fn likes(&self) -> u32 {
        self.likes
    }

    pub fn reviews(&self) -> u32 {
        self.reviews
    }

    pub fn stars(&self) -> f32 {
        self.stars
    }
}
