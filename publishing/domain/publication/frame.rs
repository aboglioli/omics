use common::result::Result;

#[derive(Debug, Clone)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(x: u32, y: u32) -> Result<Self> {
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
    pub fn new(w: u32, h: u32) -> Result<Self> {
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
    pub fn new(order: u32, position: Position, size: Size) -> Result<Self> {
        Ok(Frame {
            order,
            position,
            size,
        })
    }

    pub fn order(&self) -> u32 {
        self.order
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
