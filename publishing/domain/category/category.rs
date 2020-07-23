use common::error::Error;
use common::model::{Entity, ID};

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

pub type CategoryID = String;

#[derive(Debug, Clone)]
pub struct Category {
    id: ID<CategoryID>,
    name: CategoryName,
}

impl Category {
    pub fn new(id: CategoryID, name: CategoryName) -> Result<Category, Error> {
        Ok(Category {
            id: ID::new(id),
            name,
        })
    }

    pub fn name(&self) -> &CategoryName {
        &self.name
    }
}

impl Entity<CategoryID> for Category {
    fn id(&self) -> &ID<CategoryID> {
        &self.id
    }
}
