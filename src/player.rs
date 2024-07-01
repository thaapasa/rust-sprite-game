use ggez::graphics::Rect;

use crate::actor::{Actor, ActorType};
use crate::animation_handler::Animation;
use crate::constants::{
    GROUND_TILE_HEIGHT, GROUND_TILE_WIDTH, PLAYER_BBOX_HEIGHT, PLAYER_BBOX_WIDTH,
    PLAYER_TILE_HEIGHT, PLAYER_TILE_WIDTH,
};
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum PlayerState {
    STANDING,
    WALKING,
    RUNNING,
    JUMPING,
}

impl PlayerState {
    fn jump_speed(&self) -> f32 {
        return match self {
            PlayerState::STANDING => 500.0,
            PlayerState::WALKING => 580.0,
            PlayerState::RUNNING => 700.0,
            PlayerState::JUMPING => 600.0,
        };
    }
}

#[derive(Debug)]
pub struct Player {
    pub actor: Actor,
    pub animation: Animation,
    pub state: PlayerState,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub grounded: bool,
}

// Speeds are pixels per second
const WALKING_SPEED: f32 = 240.0;
const RUNNING_SPEED: f32 = 360.0;
const JUMP_VELOCITY: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const MAX_VELOCITY_Y: f32 = 1200.0;

impl Player {
    pub fn create() -> Player {
        let x = 5.0 * GROUND_TILE_WIDTH;
        let y = 5.0 * GROUND_TILE_HEIGHT;
        let bbox = Dimensions::new(PLAYER_BBOX_WIDTH, PLAYER_BBOX_HEIGHT);
        return Player {
            actor: Actor {
                tag: ActorType::Player,
                pos: Point2::new(x, y),
                facing: Direction::Right,
                sprite_size: Dimensions::new(PLAYER_TILE_WIDTH, PLAYER_TILE_HEIGHT),
                draw_offset: Point2::new((bbox.x - PLAYER_TILE_WIDTH) / 2.0, 0.0),
                bbox: Rect {
                    x,
                    y,
                    w: bbox.x,
                    h: bbox.y,
                },
            },
            animation: Animation::player_idle(),
            state: PlayerState::STANDING,
            velocity_x: 0.0,
            velocity_y: 0.0,
            grounded: true,
        };
    }

    pub fn handle_input(&mut self, input: &InputState, seconds: f32, level: &LevelHandler) {
        self.update_player_action(input);
        self.calc_player_pos(seconds, level);
    }

    fn update_player_action(&mut self, input: &InputState) {
        if input.jump {
            return self.jump();
        }
        match input.move_x() {
            Some(dir) => {
                self.actor.facing = dir;
                if input.running {
                    return self.run(dir);
                } else {
                    return self.walk(dir);
                }
            }
            None => (),
        }
        return self.idle();
    }

    fn idle(&mut self) {
        self.velocity_x = 0.0;
        if !matches!(self.state, PlayerState::STANDING) && self.grounded {
            self.state = PlayerState::STANDING;
            self.animation = Animation::player_idle();
        }
    }

    fn walk(&mut self, direction: Direction) {
        self.velocity_x = direction.mult() * WALKING_SPEED;
        self.actor.facing = direction;
        if !matches!(self.state, PlayerState::WALKING) && self.grounded {
            self.state = PlayerState::WALKING;
            self.animation = Animation::player_walking();
        }
    }

    fn run(&mut self, direction: Direction) {
        self.actor.facing = direction;
        self.velocity_x = direction.mult() * RUNNING_SPEED;
        if !matches!(self.state, PlayerState::RUNNING) && self.grounded {
            self.state = PlayerState::RUNNING;
            self.animation = Animation::player_running();
        }
    }

    fn jump(&mut self) {
        if !matches!(self.state, PlayerState::JUMPING) && self.grounded {
            self.velocity_y += self.state.jump_speed();
            self.state = PlayerState::JUMPING;
            self.animation = Animation::player_jumping();
        }
    }

    fn calc_player_pos(&mut self, seconds: f32, level: &LevelHandler) {
        // Update gravity
        self.velocity_y -= GRAVITY * seconds.min(MAX_VELOCITY_Y);
        // Move player along x
        self.move_by(self.velocity_x * seconds, 0.0);
        // Check for collision on x-axis
        self.check_collision(true, level);
        // Move player along y
        self.move_by(0.0, self.velocity_y * seconds);
        // Check for collision on x-axis
        self.check_collision(false, level);

        self.grounded = self.is_grounded();
        if self.grounded {
            self.velocity_y = 0.0;
        }
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.actor.pos.x += x;
        self.actor.pos.y += y;
        self.actor.update_bbox();
    }

    fn check_collision(&self, along_x: bool, level: &LevelHandler) {
        let collisions = level.get_collisions(&self.actor.bbox);
    }

    fn is_grounded(&self) -> bool {
        return true;
    }
}
