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
}
