use ggez::glam::Vec2;
use ggez::graphics::Rect;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn mult(&self) -> f32 {
        return match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        };
    }
}

pub type Point2 = Vec2;
pub type Dimensions = Vec2;

pub trait RectExt {
    fn collides_with(&self, other: &Rect) -> bool;
}

impl RectExt for Rect {
    fn collides_with(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}
