use crate::primitives::{Direction, Point2, Vector2};

#[derive(Debug)]
pub enum ActorType {
    Player,
    GroundBlock,
}

#[derive(Debug)]
pub struct Actor {
    tag: ActorType,
    pos: Point2,
    facing: Direction,
    velocity: Vector2,
}

impl Actor {
    pub fn create_player() -> Actor {
        Actor {
            tag: ActorType::Player,
            pos: Point2::new(0.0,0.0),
            facing: Direction::Left,
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}
