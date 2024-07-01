use std::fmt;
use std::fmt::Formatter;

use ggez::glam::Vec2;
use ggez::graphics::{Image, Rect};

use crate::constants::SCREEN_HEIGHT;
use crate::game::SpriteGame;
use crate::level_handler::TileType;
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum ActorType {
    Player,
    GroundBlock { x: usize, y: usize },
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ActorType::Player => write!(f, "Player"),
            ActorType::GroundBlock { x, y } => write!(f, "Ground({},{})", x, y),
        }
    }
}

/// Regarding game space:
/// Game space is pixels. Y coordinate axis direction is inverted from the draw direction
/// to make game physics more intuitive (-y is down).
///
///  +y
///  ^
///  |
///  |
///  |
///  +--------> +x
/// (0,0) is bottom left corner of the game screen.
/// Actor position is the bottom left corner of the actor's bounding box.

#[derive(Debug)]
pub struct Actor {
    pub tag: ActorType,
    /// Current position of the actor's bounding box's bottom left corner in game space.q
    /// Note: If bbox_offset is not {0,0}, the sprite will be drawn to a different location.
    pub pos: Point2,
    pub facing: Direction,
    /// Sprite draw size, in game space.
    pub sprite_size: Dimensions,
    /// Size of the actor's bounding box.
    pub bbox_size: Dimensions,
    /// Sprite offset from actor's bounding box, relative to the actor's position.
    /// {0,0} means that the bounding box starts at the same place where
    /// the actor sprite is drawn to.
    pub draw_offset: Point2,
}
impl Actor {
    pub fn create_tile(tile: &TileType, x: f32, y: f32) -> Actor {
        let bbox = Dimensions::new(32.0, 32.0);
        Actor {
            tag: ActorType::GroundBlock {
                x: tile.x,
                y: tile.y,
            },
            pos: Point2::new(x, y),
            facing: Direction::Left,
            sprite_size: Dimensions::new(32.0, 32.0),
            bbox_size: bbox,
            draw_offset: Point2::new(0.0, 0.0),
        }
    }

    pub fn screen_coords(&self, scale: &Vec2) -> Point2 {
        let x = self.pos.x + self.draw_offset.x;
        let y = SCREEN_HEIGHT - (self.pos.y + self.draw_offset.y) - self.sprite_size.y;
        Point2::new((x * scale.x).round(), (y * scale.y).round())
    }

    pub fn draw_rect(&self) -> Rect {
        return Rect {
            x: self.pos.x + self.draw_offset.x,
            y: self.pos.y + self.draw_offset.y,
            w: self.sprite_size.x,
            h: self.sprite_size.y,
        };
    }

    pub fn bbox_rect(&self) -> Rect {
        return Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: self.bbox_size.x,
            h: self.bbox_size.y,
        };
    }

    /// Returns the offset of this tile to draw this actor from its tile image
    pub fn tile_offset(&self, image: &Image, game: &SpriteGame) -> Rect {
        let x_size = self.sprite_size.x / image.width() as f32;
        let y_size = self.sprite_size.y / image.height() as f32;
        match self.tag {
            ActorType::Player => Rect {
                x: game.player.animation.get_current_frame() as f32 * x_size,
                y: 0.0,
                w: x_size,
                h: y_size,
            },
            ActorType::GroundBlock { x, y } => Rect {
                x: x as f32 * x_size,
                y: y as f32 * y_size,
                w: x_size,
                h: y_size,
            },
        }
    }
}
