use std::collections::HashMap;

use ggez::GameResult;
use ggez::input::keyboard::{KeyCode, KeyInput};

use crate::primitives::Direction;

#[derive(Debug)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub request_quit: bool,
    pub running: bool,
    raw_keys: HashMap<KeyCode, bool>,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: false,
            right: false,
            jump: false,
            request_quit: false,
            running: false,
            raw_keys: HashMap::new(),
        }
    }
}

impl InputState {
    pub fn move_x(&self) -> Option<Direction> {
        match (self.left, self.right) {
            (false, true) => Some(Direction::Right),
            (true, false) => Some(Direction::Left),
            _ => None,
        }
    }

    pub fn handle_key_down(&mut self, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(key) => {
                self.raw_keys.insert(key, true);
                self.update_state();
            }
            _ => (),
        }
        Ok(())
    }

    pub fn handle_key_up(&mut self, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(key) => {
                self.raw_keys.remove(&key);
                self.update_state();
            }
            _ => (),
        }
        Ok(())
    }

    fn update_state(&mut self) {
        self.left =
            self.raw_keys.contains_key(&KeyCode::Left) || self.raw_keys.contains_key(&KeyCode::A);
        self.right =
            self.raw_keys.contains_key(&KeyCode::Right) || self.raw_keys.contains_key(&KeyCode::D);
        self.jump = self.raw_keys.contains_key(&KeyCode::Space)
            || self.raw_keys.contains_key(&KeyCode::Up)
            || self.raw_keys.contains_key(&KeyCode::W);
        self.running = self.raw_keys.contains_key(&KeyCode::LShift)
            || self.raw_keys.contains_key(&KeyCode::RShift);
        self.request_quit =
            self.raw_keys.contains_key(&KeyCode::Escape) || self.raw_keys.contains_key(&KeyCode::Q);
    }
}
