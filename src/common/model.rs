use chrono::{DateTime, Utc};
use std::cmp::PartialEq;

// ID
#[derive(Debug, Clone)]
pub struct ID<T> {
    id: T,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl<T> ID<T>
where
    T: PartialEq + Clone,
{
    pub fn new(id: T) -> ID<T> {
        ID {
            id,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn value(&self) -> T {
        self.id.clone()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
    }

    pub fn updated(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn deleted(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

impl<T: PartialEq> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Entity
pub trait Entity<I>
where
    I: PartialEq + Clone,
{
    fn id(&self) -> &ID<I>;

    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn eq_id(&self, id: I) -> bool {
        self.id() == &ID::new(id)
    }
}

// Simple ValueObject
pub trait Value<T> {
    fn value(&self) -> T;
}
