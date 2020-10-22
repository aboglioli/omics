pub struct Pagination<T> {
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
    pub matching_criteria: usize,
    pub count: usize,
    pub items: Vec<T>,
}

impl<T> Pagination<T> {
    pub fn new(offset: usize, limit: usize, total: usize, matching_criteria: usize) -> Self {
        Pagination {
            offset,
            limit,
            total,
            matching_criteria,
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

    pub fn matching_criteria(&self) -> usize {
        self.matching_criteria
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
