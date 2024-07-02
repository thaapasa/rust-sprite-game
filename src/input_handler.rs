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
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: false,
            right: false,
            jump: false,
            request_quit: false,
            running: false,
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
            Some(KeyCode::Space) => {
                self.jump = true;
            }
            Some(KeyCode::Left) => {
                self.left = true;
            }
            Some(KeyCode::Right) => {
                self.right = true;
            }
            Some(KeyCode::LShift | KeyCode::RShift) => {
                self.running = true;
            }
            Some(KeyCode::Escape | KeyCode::Q) => self.request_quit = true,
            _ => (), // Do nothing
        }
        Ok(())
    }

    pub fn handle_key_up(&mut self, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) => {
                self.jump = false;
            }
            Some(KeyCode::LShift | KeyCode::RShift) => {
                self.running = false;
            }
            Some(KeyCode::Left) => {
                self.left = false;
            }
            Some(KeyCode::Right) => {
                self.right = false;
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}
