use ggez::{Context, GameResult};
use ggez::graphics::Image;

use crate::actor::{Actor, ActorType};
use crate::game::SpriteGame;

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
    pub player_walk_tiles: Image,
    pub player_run_tiles: Image,
    pub ground_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        Ok(GameAssets {
            background: Image::from_path(ctx, "/background.png")?,
            player_idle_tiles: Image::from_path(ctx, "/idle-tileset.png")?,
            player_run_tiles: Image::from_path(ctx, "/run-tileset.png")?,
            player_walk_tiles: Image::from_path(ctx, "/walk-tileset.png")?,
            ground_tiles: Image::from_path(ctx, "/background-tileset.png")?,
        })
    }

    pub fn actor_image(&self, actor: &Actor, game: &SpriteGame) -> &Image {
        match actor.tag {
            ActorType::Player => (game.player.animation.tileset_image)(&self),
            ActorType::GroundBlock { x, y } => &self.ground_tiles,
        }
    }
}
