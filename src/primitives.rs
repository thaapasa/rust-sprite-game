use ggez::glam::Vec2;

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
