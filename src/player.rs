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
}

impl Player {
    pub fn create() -> Player {
        let bbox = Dimensions::new(42.0, 74.0);
        return Player {
            actor: Actor {
                tag: ActorType::Player,
                pos: Point2::new(5.0 * 32.0, 2.0 * 32.0),
                facing: Direction::Left,
                sprite_size: Dimensions::new(128.0, 128.0),
                bbox_size: bbox,
                draw_offset: Point2::new((bbox.x - 128.0) / 2.0, 0.0),
            },
            animation: Animation::player_idle(),
            state: PlayerState::STANDING,
        };
    }

    pub fn handle_input(&mut self, input: &InputState, seconds: f32) {
        self.update_player_action(input);
        self.move_player(seconds);
    }

    fn update_player_action(&mut self, input: &InputState) {
        match input.move_x {
            None => {
                if !matches!(self.state, PlayerState::STANDING) {
                    self.state = PlayerState::STANDING;
                    self.animation = Animation::player_idle();
                }
            }
            Some(dir) => {
                self.actor.facing = dir;
                if input.running {
                    if !matches!(self.state, PlayerState::RUNNING) {
                        self.state = PlayerState::RUNNING;
                        self.animation = Animation::player_running();
                    }
                } else {
                    if !matches!(self.state, PlayerState::WALKING) {
                        self.state = PlayerState::WALKING;
                        self.animation = Animation::player_walking();
                    }
                }
            }
        }
    }

    fn move_player(&mut self, seconds: f32) {
        let dir_mult = match self.actor.facing {
            Direction::Left => -1.0,
            _ => 1.0,
        };
        match self.state {
            PlayerState::WALKING => {
                self.actor.pos += Point2::new(seconds * 100.0 * dir_mult, 0.0);
            }
            PlayerState::RUNNING => {
                self.actor.pos += Point2::new(seconds * 150.0 * dir_mult, 0.0);
            }
            _ => (),
        }
    }
}
