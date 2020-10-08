use serde::Deserialize;

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
