use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct Pagination<T> {
    offset: usize,
    limit: usize,
    total: usize,
    count: usize,
    results: Vec<T>,
}

impl<T> Pagination<T> {
    pub fn new(offset: usize, limit: usize, total: usize) -> Self {
        let config = Config::get();

        Pagination {
            offset,
            limit: if limit < config.pagination_limit() {
                limit
            } else {
                config.pagination_limit()
            },
            total,
            count: 0,
            results: Vec::new(),
        }
    }

    pub fn add_results(mut self, results: Vec<T>) -> Self {
        self.results.extend(results);
        self.count = self.results.len();
        self
    }

    pub fn add_result(mut self, result: T) -> Self {
        self.results.push(result);
        self.count = self.results.len();
        self
    }
}
