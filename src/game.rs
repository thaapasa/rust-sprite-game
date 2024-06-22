use ggez::{Context, GameResult};

use crate::actor::Actor;
use crate::game_assets::GameAssets;
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;

pub struct SpriteGame {
    pub assets: GameAssets,
    pub player: Actor,
    pub input: InputState,
    pub level: LevelHandler,
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> GameResult<SpriteGame> {
        let assets = GameAssets::new(ctx).expect("Could not initialize Game Assets");

        let player = Actor::create_player();
        let input = InputState::default();
        let level = LevelHandler::new("level.txt", 40, 23)?;

        Ok(SpriteGame {
            assets,
            player,
            input,
            level,
        })
    }

    pub fn actors(&self) -> Vec<&Actor> {
        let mut actors = self.level.collect_actors();
        actors.push(&self.player);
        return actors;
    }
}
