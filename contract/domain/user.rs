use common::model::StringId;
use common::result::Result;

pub type UserId = StringId;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Result<Self> {
        Ok(User { id })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }
}
