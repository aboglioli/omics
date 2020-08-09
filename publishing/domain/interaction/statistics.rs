use common::result::Result;

#[derive(Default)]
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
    ) -> Result<Statistics> {
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
}
