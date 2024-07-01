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
            grounded: true,
        };
    }

    pub fn handle_input(&mut self, input: &InputState, seconds: f32) {
        self.update_player_action(input);
        self.move_player(seconds);
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
        self.idle();
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

    fn move_player(&mut self, seconds: f32) {
        // Update gravity
        self.velocity_y -= GRAVITY * seconds.min(MAX_VELOCITY_Y);
        // Move player along x
        self.actor.pos.x += self.velocity_x * seconds;
        // Check for collision on x-axis
        self.check_collision(true);
        // Move player along y
        self.actor.pos.y += self.velocity_y * seconds;
        // Check for collision on x-axis
        self.check_collision(false);

        self.grounded = self.is_grounded();
        if self.grounded {
            self.velocity_y = 0.0;
        }
    }

    fn check_collision(&self, along_x: bool) {}

    fn is_grounded(&self) -> bool {
        return true;
    }
}
