use crate::primitives::{Direction, Point2};

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
}

impl Actor {
    pub fn create_player() -> Actor {
        Actor {
            tag: ActorType::Player,
            pos: Point2::new(300.0, 200.0),
            facing: Direction::Left,
        }
    }
}
