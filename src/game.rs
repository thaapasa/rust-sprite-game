use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::DrawParam;

use crate::actor::Actor;
use crate::game_assets::GameAssets;
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;
use crate::player::Player;
use crate::primitives::{Direction, Point2};

pub struct SpriteGame {
    pub assets: GameAssets,
    pub player: Player,
    pub input: InputState,
    pub level: LevelHandler,
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> GameResult<SpriteGame> {
        let assets = GameAssets::new(ctx).expect("Could not initialize Game Assets");

        let player = Player::create();
        let input = InputState::default();
        let level = LevelHandler::new("level.txt", 40, 23)?;

        Ok(SpriteGame {
            assets,
            player,
            input,
            level,
        })
    }

    pub fn traverse_actors<F>(&self, mut callback: F)
    where
        F: FnMut(&Actor),
    {
        self.level.traverse_actors(&mut callback);
        callback(&self.player.actor);
    }

    pub fn draw_actor(&self, actor: &Actor, canvas: &mut graphics::Canvas, scale: Vec2) {
        let img = self.assets.actor_image(actor, &self);
        let src = actor.tile_offset(&img, &self);
        let dest = actor.screen_coords(&scale);
        let params = DrawParam::new().src(src).dest(dest);

        let facing = match actor.facing {
            Direction::Left => params
                .scale(Vec2::new(-scale.x, scale.y))
                .offset(Point2::new(1.0, 0.0)),
            _ => params.scale(scale),
        };

        canvas.draw(img, facing)
    }
}
