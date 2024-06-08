mod constants;
mod primitives;
mod game;
mod game_assets;
mod actor;
mod event_handler;

use crate::game::SpriteGame;

use ggez::{conf, ContextBuilder};
use ggez::event::{self, EventHandler};
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("sprite_game", "Tuukka Haapasalo")
        .window_setup(conf::WindowSetup::default().title("Sprite Knight"))
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
