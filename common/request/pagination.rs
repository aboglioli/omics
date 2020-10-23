use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::model::Pagination;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub order_by: Option<String>,
}

impl PaginationParams {
    pub fn offset(&self) -> Option<usize> {
        self.offset.or_else(|| Some(0))
    }

    pub fn limit(&self) -> Option<usize> {
        let config = Config::get();

        self.limit
            .map(|limit| {
                if limit <= config.pagination_limit() {
                    limit
                } else {
                    config.pagination_limit()
                }
            })
            .or_else(|| Some(config.pagination_limit()))
    }

    pub fn order_by(&self) -> Option<String> {
        self.order_by.clone()
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        let config = Config::get();

        PaginationParams {
            offset: Some(0),
            limit: Some(config.pagination_limit()),
            order_by: None,
        }
    }
}

#[derive(Serialize)]
pub struct PaginationResponse<T>
where
    T: Serialize,
{
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
    pub matching_criteria: usize,
    pub count: usize,
    pub items: Vec<T>,
}

impl<T> PaginationResponse<T>
where
    T: Serialize,
{
    pub fn new(offset: usize, limit: usize, total: usize, matching_criteria: usize) -> Self {
        PaginationResponse {
            offset,
            limit,
            total,
            matching_criteria,
            count: 0,
            items: Vec::new(),
        }
    }

    pub fn add_items(&mut self, items: Vec<T>) {
        self.items.extend(items);
        self.count = self.items.len();
    }

    pub fn add_item(&mut self, result: T) {
        self.items.push(result);
        self.count = self.items.len();
    }
}

impl<T1, T2> From<&Pagination<T1>> for PaginationResponse<T2>
where
    T2: Serialize,
{
    fn from(p: &Pagination<T1>) -> Self {
        PaginationResponse::new(p.offset(), p.limit(), p.total(), p.matching_criteria())
    }
}
