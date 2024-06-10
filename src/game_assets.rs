use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawParam, Image, Rect};
use crate::actor::Actor;
use crate::constants::{SPRITE_HEIGHT, SPRITE_WIDTH};
use crate::primitives::Point2;

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        let player_idle_tiles = Image::from_path(ctx, "/idle-tileset.png")?;
        Ok(GameAssets { background, player_idle_tiles })
    }

    pub fn actor_image(&self, actor: &Actor) -> &Image {
        &self.player_idle_tiles
    }

    pub fn draw_actor(&self, actor: &Actor, canvas: &mut graphics::Canvas, offset: u8, sprite_count: u8) {
        let sprite_wid = 1.0 / sprite_count as f32;
        let offs = offset as f32 * sprite_wid;
        let params = DrawParam::new()
            .src(Rect::new(offs,0.0, offs + sprite_wid, 1.0));

        canvas.draw(self.actor_image(actor), params)
    }

}
