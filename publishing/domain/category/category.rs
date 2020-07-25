use common::error::Error;
use common::model::AggregateRoot;

#[derive(Debug, Clone)]
pub struct CategoryName {
    name: String,
}

impl CategoryName {
    pub fn new(name: &str) -> Result<CategoryName, Error> {
        Ok(CategoryName {
            name: name.to_owned(),
        })
    }

    pub fn value(&self) -> &str {
        &self.name
    }
}

pub type CategoryId = String;

#[derive(Debug, Clone)]
pub struct Category {
    base: AggregateRoot<CategoryId>,
    name: CategoryName,
}

impl Category {
    pub fn new(id: CategoryId, name: CategoryName) -> Result<Category, Error> {
        Ok(Category {
            base: AggregateRoot::new(id),
            name,
        })
    }

    pub fn name(&self) -> &CategoryName {
        &self.name
    }
}
