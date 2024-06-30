use ggez::{Context, GameResult};
use ggez::graphics::Image;

use crate::actor::{Actor, ActorType};
use crate::game::SpriteGame;

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
    pub ground_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        let player_idle_tiles = Image::from_path(ctx, "/idle-tileset.png")?;
        let ground_tiles = Image::from_path(ctx, "/background-tileset.png")?;
        Ok(GameAssets {
            background,
            player_idle_tiles,
            ground_tiles,
        })
    }

    pub fn actor_image(&self, actor: &Actor, game: &SpriteGame) -> &Image {
        match actor.tag {
            ActorType::Player => (game.player_animation.tileset_image)(self),
            ActorType::GroundBlock { x, y } => &self.ground_tiles,
        }
    }
}
