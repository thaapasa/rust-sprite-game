use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawParam, Image, Rect};

use crate::actor::Actor;

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        let player_idle_tiles = Image::from_path(ctx, "/idle-tileset.png")?;
        Ok(GameAssets {
            background,
            player_idle_tiles,
        })
    }

    pub fn actor_image(&self, _actor: &Actor) -> &Image {
        &self.player_idle_tiles
    }

    pub fn draw_actor(
        &self,
        actor: &Actor,
        canvas: &mut graphics::Canvas,
        offset: u8,
        sprite_count: u8,
    ) {
        let sprite_wid = 1.0 / sprite_count as f32;
        let tile_offs = offset as f32 * sprite_wid;
        let pos = actor.screen_coords();
        let params = DrawParam::new()
            .src(Rect::new(tile_offs, 0.0, tile_offs + sprite_wid, 1.0))
            .dest(pos);

        canvas.draw(self.actor_image(actor), params)
    }
}
