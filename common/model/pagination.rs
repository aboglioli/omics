use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct Pagination<T> {
    offset: usize,
    limit: usize,
    total: usize,
    count: usize,
    items: Vec<T>,
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
            items: Vec::new(),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    pub fn add_items(mut self, items: Vec<T>) -> Self {
        self.items.extend(items);
        self.count = self.items.len();
        self
    }

    pub fn add_item(mut self, result: T) -> Self {
        self.items.push(result);
        self.count = self.items.len();
        self
    }
}
