use chrono::{DateTime, Utc};

// TODO: not used
pub struct TimeOrderedDataItem<T> {
    date: DateTime<Utc>,
    data: T,
}

impl<T> TimeOrderedDataItem<T> {
    pub fn new(data: T) {
        TimeOrderedDataItem {
            date: Utc::now(),
            data,
        }
    }
}

pub struct TimeOrderedData<T> {
    items: Vec<TimeOrderedDataItem<T>>,
}

impl<T> TimeOrderedData<T> {
    pub fn new() -> Self {
        TimeOrderedData {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.items.push(TimeOrderedDataItem::new(data));
    }
}
