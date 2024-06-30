use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawParam};
use ggez::input::keyboard::KeyInput;

use crate::actor::Actor;
use crate::constants::DESIRED_FPS;
use crate::game::SpriteGame;
use crate::input_handler::InputState;
use crate::primitives::{Direction, Point2};

impl EventHandler for SpriteGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            if self.input.request_quit {
                println!("Quitting game...");
                ctx.request_quit()
            }

            player_handle_input(&mut self.player, &self.input, seconds);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let scale_factor = ctx.gfx.window().scale_factor() as f32;
        let scale = Vec2 {
            x: scale_factor,
            y: scale_factor,
        };
        canvas.draw(&self.assets.background, DrawParam::new().scale(scale));
        self.traverse_actors(|a| self.assets.draw_actor(a, &mut canvas, 0, scale));
        // Draw code here...
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        self.input.handle_key_down(input)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        self.input.handle_key_up(input)
    }
}

fn player_handle_input(actor: &mut Actor, input: &InputState, seconds: f32) {
    match input.move_x {
        None => (),
        Some(Direction::Left) => {
            actor.facing = Direction::Left;
            actor.pos += Point2::new(-seconds * 100.0, 0.0)
        }
        Some(Direction::Right) => {
            actor.facing = Direction::Right;
            actor.pos += Point2::new(seconds * 100.0, 0.0)
        }
    }
}
