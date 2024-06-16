use crate::constants::{SCREEN_HEIGHT, SPRITE_HEIGHT, SPRITE_WIDTH};
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum ActorType {
    Player,
    GroundBlock,
}

#[derive(Debug)]
pub struct Actor {
    tag: ActorType,
    pub pos: Point2,
    pub facing: Direction,
    pub sprite_size: Dimensions,
    pub bbox_size: Dimensions,
    pub bbox_offset: Point2,
}

impl Actor {
    pub fn create_player() -> Actor {
        let bbox = Dimensions::new(42.0, 74.0);
        Actor {
            tag: ActorType::Player,
            pos: Point2::new(20.0, 20.0),
            facing: Direction::Left,
            sprite_size: Dimensions::new(SPRITE_WIDTH, SPRITE_HEIGHT),
            bbox_size: bbox,
            bbox_offset: Point2::new((SPRITE_WIDTH - bbox.x) / 2.0, 0.0),
        }
    }

    pub fn screen_coords(&self) -> Point2 {
        let x = self.pos.x - self.bbox_offset.x;
        let y = SCREEN_HEIGHT - (self.pos.y - self.bbox_offset.y) - SPRITE_HEIGHT;
        Point2::new(x.round(), y.round())
    }
}
