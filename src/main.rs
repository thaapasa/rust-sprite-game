mod constants;
mod primitives;
mod game;
mod game_assets;

use crate::game::SpriteGame;

use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("sprite_game", "Tuukka Haapasalo")
        .window_setup(conf::WindowSetup::default().title("Sprite game"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("resources")
        .build()
        .expect("Could not create ggez context!");
    println!("Game resource path: {:?}", ctx.fs);

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = SpriteGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

impl EventHandler for SpriteGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.draw(&self.assets.background, DrawParam::new());
        // Draw code here...
        canvas.finish(ctx)
    }
}