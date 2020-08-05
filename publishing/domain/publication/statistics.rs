pub struct Statistics {
    likes: u32,
    views: u32,
    unique_views: u32,
    readings: u32,
}

impl Statistics {
    pub fn new(likes: u32, views: u32, unique_views: u32, readings: u32) -> Statistics {
        Statistics {
            likes,
            views,
            unique_views,
            readings,
        }
    }

    pub fn likes(&self) -> u32 {
        self.likes
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
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            likes: 0,
            views: 0,
            unique_views: 0,
            readings: 0,
        }
    }
}
