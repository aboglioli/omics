pub struct Statistics {
    likes: u32,
    views: u32,
    unique_views: u32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            likes: 0,
            views: 0,
            unique_views: 0,
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
}
