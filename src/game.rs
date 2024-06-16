use ggez::Context;

use crate::actor::Actor;
use crate::game_assets::GameAssets;
use crate::input_handler::InputState;

pub struct SpriteGame {
    pub assets: GameAssets,
    pub player: Actor,
    pub input: InputState,
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> SpriteGame {
        let assets = GameAssets::new(ctx).expect("Could not initialize Game Assets");

        let player = Actor::create_player();
        let input = InputState::default();

        SpriteGame {
            assets,
            player,
            input,
        }
    }
}
