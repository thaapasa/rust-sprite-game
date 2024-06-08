use ggez::{Context, GameResult};
use ggez::graphics::Image;
use crate::actor::Actor;

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        let player_idle_tiles = Image::from_path(ctx, "/idle-tileset.png")?;
        Ok(GameAssets { background, player_idle_tiles })
    }

    pub fn actor_image(&self, actor: &Actor) -> &Image {
        &self.player_idle_tiles
    }
}
