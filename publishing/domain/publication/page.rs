use common::result::Result;

use crate::domain::publication::Image;

#[derive(Debug, Clone)]
pub struct Page {
    number: u32,
    images: Vec<Image>,
}

impl Page {
    pub fn new(number: u32) -> Result<Self> {
        Ok(Page {
            number,
            images: Vec::new(),
        })
    }

    pub fn with_images(number: u32, images: Vec<Image>) -> Result<Self> {
        Ok(Page { number, images })
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn images(&self) -> &[Image] {
        &self.images
    }

    pub fn set_images(&mut self, images: Vec<Image>) -> Result<()> {
        self.images = images;
        Ok(())
    }
}
