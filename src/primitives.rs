use ggez::glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub type Point2 = Vec2;
pub type Dimensions = Vec2;
