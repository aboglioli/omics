use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub offset: usize,
    pub limit: usize,
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams {
            offset: 0,
            limit: 100,
        }
    }
}
