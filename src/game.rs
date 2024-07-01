use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam};

use crate::actor::Actor;
use crate::constants::DRAW_BBOX;
use crate::game_gfx::GraphicsHandler;
use crate::input_handler::InputState;
use crate::level_handler::LevelHandler;
use crate::player::Player;

pub struct SpriteGame {
    pub player: Player,
    pub input: InputState,
    pub level: LevelHandler,
    pub gfx: GraphicsHandler,
}

impl SpriteGame {
    pub fn new(ctx: &mut Context) -> GameResult<SpriteGame> {
        let player = Player::create();
        let input = InputState::default();
        let level = LevelHandler::new("level.txt", 40, 23)?;
        let gfx = GraphicsHandler::new(ctx)?;

        Ok(SpriteGame {
            player,
            input,
            level,
            gfx,
        })
    }

    pub fn traverse_actors<F>(&self, mut callback: F)
    where
        F: FnMut(&Actor),
    {
        self.level.traverse_actors(&mut callback);
        callback(&self.player.actor);
    }

    pub fn draw_frame(&mut self, canvas: &mut Canvas, scale: Vec2) {
        canvas.draw(&self.gfx.assets.background, DrawParam::new().scale(scale));
        self.traverse_actors(|a| self.gfx.draw_actor(a, canvas, &self, scale));

        if DRAW_BBOX {
            self.traverse_actors(|a| self.gfx.draw_bbox(a, canvas, scale));
        }
    }
}
