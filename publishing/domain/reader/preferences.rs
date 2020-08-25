use crate::domain::category::CategoryId;

#[derive(Debug, Clone)]
pub struct Preferences {
    categories: Vec<CategoryId>,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            categories: Vec::new(),
        }
    }
}
