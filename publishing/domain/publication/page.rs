use common::result::Result;

use crate::domain::publication::Image;

pub type PageNumber = u32;

#[derive(Debug, Clone)]
pub struct Page {
    number: PageNumber,
    images: Vec<Image>,
}

impl Page {
    pub fn new(number: u32) -> Result<Self> {
        Ok(Page {
            number,
            images: Vec::new(),
        })
    }

    pub fn number(&self) -> &PageNumber {
        &self.number
    }

    pub fn images(&self) -> &[Image] {
        &self.images
    }

    pub fn set_images(&mut self, images: Vec<Image>) -> Result<()> {
        self.images = images;
        Ok(())
    }
}
