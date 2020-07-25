use std::cmp::PartialEq;

use common::error::Error;

#[derive(Debug, Clone)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(x: u32, y: u32) -> Result<Position, Error> {
        Ok(Position(x, y))
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug, Clone)]
pub struct Size(u32, u32);

impl Size {
    pub fn new(w: u32, h: u32) -> Result<Size, Error> {
        Ok(Size(w, h))
    }

    pub fn width(&self) -> u32 {
        self.0
    }

    pub fn height(&self) -> u32 {
        self.1
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    order: u32,
    position: Position,
    size: Size,
}

impl Frame {
    pub fn new(order: u32, position: Position, size: Size) -> Result<Frame, Error> {
        Ok(Frame {
            order,
            position,
            size,
        })
    }

    pub fn order(&self) -> &u32 {
        &self.order
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

pub type ImageId = String;

pub type ImageURL = String;

#[derive(Debug, Clone)]
pub struct Image {
    id: ImageId,
    url: ImageURL,
    size: u32,
    frames: Vec<Frame>,
}

impl Image {
    pub fn new(id: ImageId, url: &str, size: u32) -> Result<Image, Error> {
        Ok(Image {
            id,
            url: url.to_owned(),
            size,
            frames: Vec::new(),
        })
    }

    pub fn id(&self) -> &ImageId {
        &self.id
    }

    pub fn url(&self) -> &ImageURL {
        &self.url
    }

    pub fn size(&self) -> &u32 {
        &self.size
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn add_frame(&mut self, frame: Frame) -> Result<(), Error> {
        for f in self.frames.iter_mut() {
            if f.order() == frame.order() {
                *f = frame;
                return Ok(());
            }
        }
        self.frames.push(frame);
        Ok(())
    }
}

pub type PageNumber = u32;

#[derive(Debug, Clone)]
pub struct Page {
    number: PageNumber,
    images: Vec<Image>,
}

impl Page {
    pub fn new(number: u32) -> Result<Page, Error> {
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

    pub fn add_image(&mut self, image: Image) -> Result<(), Error> {
        for i in self.images.iter_mut() {
            if i.id() == image.id() {
                *i = image;
                return Ok(());
            }
        }

        self.images.push(image);
        Ok(())
    }

    pub fn remove_image(&mut self, image_id: &ImageId) -> Result<(), Error> {
        self.images.retain(|image| image.id() != image_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image() -> Result<(), Error> {
        // New image
        let mut image = Image::new(ImageId::from("image123"), "host.com/image.jpg", 1024)?;
        image.add_frame(Frame::new(0, Position::new(0, 0)?, Size::new(800, 600)?)?)?;
        image.add_frame(Frame::new(1, Position::new(800, 0)?, Size::new(800, 600)?)?)?;
        image.add_frame(Frame::new(
            2,
            Position::new(1600, 0)?,
            Size::new(800, 600)?,
        )?)?;

        assert_eq!(image.frames().len(), 3);
        assert_eq!(image.frames()[0].order(), &0);
        assert_eq!(image.frames()[1].order(), &1);
        assert_eq!(image.frames()[2].order(), &2);

        // Replace frame
        image.add_frame(Frame::new(
            1,
            Position::new(600, 600)?,
            Size::new(600, 600)?,
        )?)?;
        assert_eq!(image.frames().len(), 3);
        assert_eq!(image.frames()[1].order(), &1);
        assert_eq!(image.frames()[1].position(), &Position::new(600, 600)?);

        Ok(())
    }
}
