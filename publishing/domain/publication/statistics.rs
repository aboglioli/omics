use serde::{Deserialize, Serialize};

use common::error::Error;
use common::result::Result;

use crate::domain::interaction::Stars;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    views: u32,
    unique_views: u32,
    readings: u32,
    likes: u32,
    reviews: u32,
    stars: f32,
}

impl Statistics {
    pub fn new(
        views: u32,
        unique_views: u32,
        readings: u32,
        likes: u32,
        reviews: u32,
        stars: f32,
    ) -> Result<Self> {
        if stars < 0.0 {
            return Err(Error::new("statistics", "stars_are_not_positive"));
        }

        Ok(Statistics {
            views,
            unique_views,
            readings,
            likes,
            reviews,
            stars,
        })
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

    pub fn add_view(&mut self, unique: bool) {
        self.views += 1;
        if unique {
            self.unique_views += 1;
        }
    }

    pub fn add_reading(&mut self) {
        self.readings += 1;
    }

    pub fn add_like(&mut self) {
        self.likes += 1;
    }

    pub fn remove_like(&mut self) {
        self.likes -= 1;
    }

    pub fn add_review(&mut self, stars: &Stars) {
        let mut total_stars = self.stars * self.reviews as f32;
        total_stars += stars.value() as f32;

        self.reviews += 1;
        self.stars = total_stars / self.reviews as f32;
    }

    pub fn remove_review(&mut self, stars: &Stars) {
        let mut total_stars = self.stars * self.reviews as f32;
        total_stars -= stars.value() as f32;

        self.reviews -= 1;
        self.stars = total_stars / self.reviews as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view() {
        let mut statistics = Statistics::default();
        statistics.add_view(true);
        statistics.add_view(true);
        statistics.add_view(true);
        statistics.add_view(false);
        statistics.add_view(false);
        assert_eq!(statistics.views(), 5);
        assert_eq!(statistics.unique_views(), 3);
    }

    #[test]
    fn prom_stars() {
        let mut statistics = Statistics::new(1000, 100, 88, 20, 5, 3.2).unwrap();

        statistics.add_review(&Stars::new(5).unwrap());
        assert_eq!(statistics.reviews(), 6);
        assert_eq!(statistics.stars(), 3.5);

        statistics.remove_review(&Stars::new(5).unwrap());
        assert_eq!(statistics.reviews(), 5);
        assert_eq!(statistics.stars(), 3.2);

        statistics.add_review(&Stars::new(2).unwrap());
        assert_eq!(statistics.reviews(), 6);
        assert_eq!(statistics.stars(), 3.0);

        statistics.add_review(&Stars::new(0).unwrap());
        assert_eq!(statistics.reviews(), 7);

        statistics.add_review(&Stars::new(0).unwrap());
        assert_eq!(statistics.reviews(), 8);
        assert_eq!(statistics.stars(), 2.25);

        statistics.add_review(&Stars::new(0).unwrap());
        assert_eq!(statistics.reviews(), 9);
        assert_eq!(statistics.stars(), 2.0);

        statistics.remove_review(&Stars::new(4).unwrap());
        assert_eq!(statistics.reviews(), 8);
        assert_eq!(statistics.stars(), 1.75);
    }
}
