use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam};
use crate::game::SpriteGame;

impl EventHandler for SpriteGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.draw(&self.assets.background, DrawParam::new());
        for a in self.actors().into_iter() {
            canvas.draw(self.assets.actor_image(a), DrawParam::new())
        }
        // Draw code here...
        canvas.finish(ctx)
    }
}