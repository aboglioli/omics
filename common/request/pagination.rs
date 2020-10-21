use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub order_by: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams {
            offset: Some(0),
            limit: Some(100),
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
