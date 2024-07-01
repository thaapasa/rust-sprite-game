use ggez::{conf, ContextBuilder};
use ggez::event::{self};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game::SpriteGame;

mod actor;
mod animation_handler;
mod constants;
mod event_handler;
mod game;
mod game_assets;
mod game_gfx;
mod input_handler;
mod level_handler;
mod player;
mod primitives;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("sprite_game", "Tuukka Haapasalo")
        .window_setup(conf::WindowSetup::default().title("Sprite Knight"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("resources")
        .build()
        .expect("Could not create ggez context!");

    // Get the scale factor
    let scale_factor = ctx.gfx.window().scale_factor();
    let new_width = SCREEN_WIDTH * scale_factor as f32;
    let new_height = SCREEN_HEIGHT * scale_factor as f32;
    ctx.gfx
        .set_drawable_size(new_width, new_height)
        .expect("Could not set screen size");

    println!("Display scale factor: {}", scale_factor);
    println!("Game resource path: {:?}", ctx.fs);

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let sprite_knight = SpriteGame::new(&mut ctx).expect("Could not initialize game");

    println!("Starting Sprite Knight game loop");

    // Run!
    event::run(ctx, event_loop, sprite_knight);
}
