use ggez::GameResult;
use ggez::input::keyboard::{KeyCode, KeyInput};

use crate::primitives::Direction;

#[derive(Debug)]
pub struct InputState {
    pub move_x: Option<Direction>,
    pub jump: bool,
    pub request_quit: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            move_x: None,
            jump: false,
            request_quit: false,
        }
    }
}

impl InputState {
    pub fn handle_key_down(&mut self, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.jump = true;
            }
            Some(KeyCode::Left) => {
                self.move_x = Some(Direction::Left);
            }
            Some(KeyCode::Right) => {
                self.move_x = Some(Direction::Right);
            }
            Some(KeyCode::Escape | KeyCode::Q) => self.request_quit = true,
            _ => (), // Do nothing
        }
        Ok(())
    }

    pub fn handle_key_up(&mut self, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.jump = false;
            }
            Some(KeyCode::Left | KeyCode::Right) => {
                self.move_x = None;
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}
