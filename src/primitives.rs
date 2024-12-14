use ggez::glam::Vec2;
use ggez::graphics::Rect;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn mult(&self) -> f32 {
        match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
}

pub type Point2 = Vec2;
pub type Dimensions = Vec2;

pub trait RectExt {
    fn collides_with(&self, other: &Rect) -> bool;
    fn clip(&self, other: &Rect) -> Option<Rect>;
}

impl RectExt for Rect {
    fn collides_with(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
    fn clip(&self, other: &Rect) -> Option<Rect> {
        let min_x = self.x.max(other.x);
        let min_y = self.y.max(other.y);
        let max_x = (self.x + self.w).min(other.x + other.w);
        let max_y = (self.y + self.h).min(other.y + other.h);
        let w = max_x - min_x;
        let h = max_y - min_y;
        if w <= 0.0 || h <= 0.0 {
            return None;
        }
        Some(Rect {
            x: min_x,
            y: min_y,
            w,
            h,
        })
    }
}
