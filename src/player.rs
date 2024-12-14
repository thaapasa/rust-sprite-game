use ggez::glam::Vec2;
use ggez::graphics::Rect;

use crate::actor::{Actor, ActorType};
use crate::animation_handler::Animation;
use crate::collision::find_mtv;
use crate::constants::{
    GROUND_TILE_HEIGHT, GROUND_TILE_WIDTH, PLAYER_BBOX_HEIGHT, PLAYER_BBOX_WIDTH,
    PLAYER_TILE_HEIGHT, PLAYER_TILE_WIDTH,
};
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;
use crate::primitives::{Dimensions, Direction, Point2};

#[derive(Debug)]
pub enum PlayerState {
    Standing,
    Walking,
    Running,
    Jumping,
}

impl PlayerState {
    fn jump_speed(&self) -> f32 {
        match self {
            PlayerState::Standing => JUMP_VELOCITY,
            PlayerState::Walking => JUMP_VELOCITY * 1.16,
            PlayerState::Running => JUMP_VELOCITY * 1.4,
            PlayerState::Jumping => JUMP_VELOCITY,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub actor: Actor,
    pub animation: Animation,
    pub state: PlayerState,
    pub velocity: Vec2,
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
        Player {
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
            state: PlayerState::Standing,
            velocity: Vec2::new(0.0, 0.0),
            grounded: true,
        }
    }

    pub fn handle_input(&mut self, input: &InputState, seconds: f32, level: &LevelHandler) {
        self.update_player_action(input);
        self.calc_player_pos(seconds, level);
    }

    fn update_player_action(&mut self, input: &InputState) {
        if input.jump {
            return self.jump();
        }
        if let Some(dir) = input.move_x() {
            self.actor.facing = dir;
            if input.running {
                return self.run(dir);
            } else {
                return self.walk(dir);
            }
        }
        self.idle()
    }

    fn idle(&mut self) {
        self.velocity.x = 0.0;
        if !matches!(self.state, PlayerState::Standing) && self.grounded {
            self.state = PlayerState::Standing;
            self.animation = Animation::player_idle();
        }
    }

    fn walk(&mut self, direction: Direction) {
        self.velocity.x = direction.mult() * WALKING_SPEED;
        self.actor.facing = direction;
        if !matches!(self.state, PlayerState::Walking) && self.grounded {
            self.state = PlayerState::Walking;
            self.animation = Animation::player_walking();
        }
    }

    fn run(&mut self, direction: Direction) {
        self.actor.facing = direction;
        self.velocity.x = direction.mult() * RUNNING_SPEED;
        if !matches!(self.state, PlayerState::Running) && self.grounded {
            self.state = PlayerState::Running;
            self.animation = Animation::player_running();
        }
    }

    fn jump(&mut self) {
        if !matches!(self.state, PlayerState::Jumping) && self.grounded {
            self.velocity.y += self.state.jump_speed();
            self.state = PlayerState::Jumping;
            self.animation = Animation::player_jumping();
        }
    }

    fn calc_player_pos(&mut self, seconds: f32, level: &LevelHandler) {
        self.velocity.y = self.velocity.y.max(-MAX_VELOCITY_Y);
        // Move player along x
        self.move_by(self.velocity.x * seconds, 0.0);
        // Check for collision on x-axis
        self.check_collision(true, level);
        // Move player along y
        self.move_by(0.0, self.velocity.y * seconds);
        // Check for collision on x-axis
        self.check_collision(false, level);
        // Update gravity (takes effect on next round, will be reset if player is grounded)
        self.velocity.y -= GRAVITY * seconds;

        self.grounded = self.is_grounded(level);
        if self.grounded {
            self.velocity.y = 0.0;
        }
    }

    fn move_by(&mut self, x: f32, y: f32) {
        self.actor.pos.x += x;
        self.actor.pos.y += y;
        self.actor.update_bbox();
    }

    fn check_collision(&mut self, along_x: bool, level: &LevelHandler) {
        let collisions = level.get_collisions(&self.actor.bbox);
        for c in collisions {
            self.resolve_collision(c, along_x);
        }
    }

    fn resolve_collision(&mut self, actor: &Actor, along_x: bool) {
        if let Some(offs) = find_mtv(&self.actor.bbox, &actor.bbox, self.velocity, along_x) {
            self.move_by(offs.x, offs.y)
        }
    }

    fn is_grounded(&self, level: &LevelHandler) -> bool {
        let ground_check = Rect {
            x: self.actor.bbox.x,
            y: self.actor.bbox.y - 1.0,
            w: self.actor.bbox.w,
            h: 1.0,
        };
        level.collides_with(&ground_check)
    }
}
