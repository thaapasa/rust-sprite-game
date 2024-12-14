use std::collections::HashMap;

use ggez::GameResult;
use ggez::input::keyboard::{KeyCode, KeyInput};

use crate::primitives::Direction;

#[derive(Debug, Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub request_quit: bool,
    pub running: bool,
    raw_keys: HashMap<KeyCode, bool>,
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
        if let Some(key) = input.keycode {
            self.raw_keys.insert(key, true);
            self.update_state();
        }
        Ok(())
    }

    pub fn handle_key_up(&mut self, input: KeyInput) -> GameResult {
        if let Some(key) = input.keycode {
            self.raw_keys.remove(&key);
            self.update_state();
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
