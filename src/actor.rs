use ggez::glam::Vec2;

use crate::constants::SCREEN_HEIGHT;
use crate::level_handler::TileType;
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum ActorType {
    Player,
    GroundBlock { x: usize, y: usize },
}

/// Regarding game space:
/// Game space is pixels since the sprites

#[derive(Debug)]
pub struct Actor {
    pub tag: ActorType,
    /// Current position of the actor in game space.
    pub pos: Point2,
    pub facing: Direction,
    /// Sprite draw size, in game space.
    pub sprite_size: Dimensions,
    /// Size of the actor's bounding box.
    pub bbox_size: Dimensions,
    /// Bounding box offset, relative to the actor's position.
    /// {0,0} means that the bounding box starts at the same place where
    /// the actor sprite is drawn to.
    pub bbox_offset: Point2,
}

impl Actor {
    pub fn create_player() -> Actor {
        let bbox = Dimensions::new(42.0, 74.0);
        Actor {
            tag: ActorType::Player,
            pos: Point2::new(20.0, 20.0),
            facing: Direction::Left,
            sprite_size: Dimensions::new(128.0, 128.0),
            bbox_size: bbox,
            bbox_offset: Point2::new((128.0 - bbox.x) / 2.0, 0.0),
        }
    }

    pub fn create_tile(tile: &TileType, x: f32, y: f32) -> Actor {
        let bbox = Dimensions::new(32.0, 32.0);
        Actor {
            tag: ActorType::GroundBlock {
                x: tile.x,
                y: tile.x,
            },
            pos: Point2::new(x, y),
            facing: Direction::Left,
            sprite_size: Dimensions::new(32.0, 32.0),
            bbox_size: bbox,
            bbox_offset: Point2::new(0.0, 0.0),
        }
    }

    pub fn screen_coords(&self, scale: &Vec2) -> Point2 {
        let x = self.pos.x - self.bbox_offset.x;
        let y = SCREEN_HEIGHT - (self.pos.y - self.bbox_offset.y) - self.sprite_size.y;
        Point2::new((x * scale.x).round(), (y * scale.y).round())
    }
}
