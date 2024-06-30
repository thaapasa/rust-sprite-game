use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::DrawParam;

use crate::actor::Actor;
use crate::animation_handler::Animation;
use crate::game_assets::GameAssets;
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;

pub struct SpriteGame {
    pub assets: GameAssets,
    pub player: Actor,
    pub player_animation: Animation,
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
            player_animation: Animation::player_idle(),
            input,
            level,
        })
    }

    pub fn traverse_actors<F>(&self, mut callback: F)
    where
        F: FnMut(&Actor),
    {
        self.level.traverse_actors(&mut callback);
        callback(&self.player);
    }

    pub fn draw_actor(&self, actor: &Actor, canvas: &mut graphics::Canvas, scale: Vec2) {
        let img = self.assets.actor_image(actor, &self);
        let src = actor.tile_offset(&img, &self);
        let dest = actor.screen_coords(&scale);
        let params = DrawParam::new().src(src).dest(dest).scale(scale);

        canvas.draw(img, params)
    }
}
