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

pub type ImageID = String;

pub type ImageURL = String;

#[derive(Debug, Clone)]
pub struct Image {
    id: ImageID,
    url: ImageURL,
    size: u32,
    frames: Vec<Frame>,
}

impl Image {
    pub fn new(id: ImageID, url: &str, size: u32) -> Result<Image, Error> {
        Ok(Image {
            id,
            url: url.to_owned(),
            size,
            frames: Vec::new(),
        })
    }

    pub fn id(&self) -> &ImageID {
        &self.id
    }

    pub fn url(&self) -> &ImageURL {
        &self.url
    }

    pub fn size(&self) -> &u32 {
        &self.size
    }

    pub fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }

    pub fn add_frame(&mut self, frame: Frame) -> Result<(), Error> {
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

    pub fn images(&self) -> &Vec<Image> {
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

    pub fn remove_image(&mut self, image_id: &ImageID) -> Result<(), Error> {
        self.images.retain(|image| image.id() != image_id);
        Ok(())
    }
}
