#[derive(Debug)]
pub enum RectangleErrors {
    NonPositive,
}

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Result<Rectangle, RectangleErrors> {
        if width <= 0 || height <= 0 {
            // Ne sert absolument à rien, juste de la gestion d'erreur pour revoir les enums
            return Err(RectangleErrors::NonPositive);
        }
        Ok(Rectangle { width, height })
    }

    pub fn area(&self) -> i32 {
        &self.width * &self.height
    }
}
