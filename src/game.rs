use ggez::Context;
use crate::actor::Actor;
use crate::game_assets::GameAssets;

pub struct SpriteGame {
    pub assets: GameAssets,
    pub player: Actor,
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> SpriteGame {
        let assets = GameAssets::new(ctx)
            .expect("Could not initialize Game Assets");

        let player = Actor::create_player();

        SpriteGame {
            assets,
            player,
        }
    }
}
