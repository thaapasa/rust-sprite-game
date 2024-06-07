use std::env;
use std::path::PathBuf;
use ggez::{Context, GameResult};
use ggez::graphics::Image;

pub struct GameAssets {
    pub background: Image
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        Ok(GameAssets { background })
    }
}
