use ggez::Context;
use crate::game_assets::GameAssets;

pub struct SpriteGame {
    pub assets: GameAssets
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> SpriteGame {
        let assets = GameAssets::new(ctx)
            .expect("Could not initialize Game Assets");

        SpriteGame {
            assets
        }
    }
}
