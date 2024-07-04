use std::fmt;
use std::fmt::Formatter;

use ggez::graphics::{Image, Rect};

use crate::constants::{GROUND_TILE_HEIGHT, GROUND_TILE_WIDTH};
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
    /// Sprite offset from actor's bounding box, relative to the actor's position.
    /// {0,0} means that the bounding box starts at the same place where
    /// the actor sprite is drawn to.
    pub draw_offset: Point2,
    /// Bounding box in game space, calculated from other fields. Used for collision calculation.
    /// Updated each frame (for player).
    pub bbox: Rect,
}
impl Actor {
    pub fn create_ground(tile: &TileType, x: usize, y: usize) -> Actor {
        let pos = Point2::new(x as f32 * GROUND_TILE_WIDTH, y as f32 * GROUND_TILE_HEIGHT);
        Actor {
            tag: ActorType::GroundBlock {
                x: tile.x,
                y: tile.y,
            },
            pos: pos,
            facing: Direction::Right,
            sprite_size: Dimensions::new(GROUND_TILE_WIDTH, GROUND_TILE_HEIGHT),
            draw_offset: Point2::new(0.0, 0.0),
            bbox: Rect {
                x: pos.x,
                y: pos.y,
                w: GROUND_TILE_WIDTH,
                h: GROUND_TILE_HEIGHT,
            },
        }
    }

    pub fn update_bbox(&mut self) {
        self.bbox.x = self.pos.x;
        self.bbox.y = self.pos.y;
        // Actor size does not vary for any actor in this game
    }

    pub fn draw_rect(&self) -> Rect {
        return Rect {
            x: self.pos.x + self.draw_offset.x,
            y: self.pos.y + self.draw_offset.y,
            w: self.sprite_size.x,
            h: self.sprite_size.y,
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
