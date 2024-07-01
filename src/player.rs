use crate::actor::{Actor, ActorType};
use crate::animation_handler::Animation;
use crate::input_handler::InputState;
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum PlayerState {
    STANDING,
    WALKING,
    RUNNING,
    JUMPING,
}

#[derive(Debug)]
pub struct Player {
    pub actor: Actor,
    pub animation: Animation,
    pub state: PlayerState,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

// Speeds are pixels per second
const WALKING_SPEED: f32 = 240.0;
const RUNNING_SPEED: f32 = 360.0;
const JUMP_VELOCITY: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const MAX_VELOCITY_Y: f32 = 1200.0;

impl Player {
    pub fn create() -> Player {
        let bbox = Dimensions::new(42.0, 74.0);
        return Player {
            actor: Actor {
                tag: ActorType::Player,
                pos: Point2::new(5.0 * 32.0, 2.0 * 32.0),
                facing: Direction::Right,
                sprite_size: Dimensions::new(128.0, 128.0),
                bbox_size: bbox,
                draw_offset: Point2::new((bbox.x - 128.0) / 2.0, 0.0),
            },
            animation: Animation::player_idle(),
            state: PlayerState::STANDING,
            velocity_x: 0.0,
            velocity_y: 0.0,
        };
    }

    pub fn handle_input(&mut self, input: &InputState, seconds: f32) {
        self.update_player_action(input);
        self.move_player(seconds);
    }

    fn update_player_action(&mut self, input: &InputState) {
        match input.move_x() {
            None => self.idle(),
            Some(dir) => {
                self.actor.facing = dir;
                if input.running {
                    self.run(dir);
                } else {
                    self.walk(dir);
                }
            }
        }
    }

    fn idle(&mut self) {
        self.velocity_x = 0.0;
        if !matches!(self.state, PlayerState::STANDING) {
            self.state = PlayerState::STANDING;
            self.animation = Animation::player_idle();
        }
    }

    fn walk(&mut self, direction: Direction) {
        if !matches!(self.state, PlayerState::WALKING) {
            self.actor.facing = direction;
            self.velocity_x = direction.mult() * WALKING_SPEED;
            self.state = PlayerState::WALKING;
            self.animation = Animation::player_walking();
        }
    }

    fn run(&mut self, direction: Direction) {
        if !matches!(self.state, PlayerState::RUNNING) {
            self.actor.facing = direction;
            self.velocity_x = direction.mult() * RUNNING_SPEED;
            self.state = PlayerState::RUNNING;
            self.animation = Animation::player_running();
        }
    }

    fn move_player(&mut self, seconds: f32) {
        if self.velocity_x != 0.0 || self.velocity_y != 0.0 {
            self.actor.pos += Point2::new(seconds * self.velocity_x, seconds * self.velocity_y);
        }
    }
}
