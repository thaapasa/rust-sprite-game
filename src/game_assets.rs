use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawParam, Image, Rect};

use crate::actor::Actor;
use crate::constants::SCREEN_HEIGHT;
use crate::primitives::Point2;

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

    pub fn actor_image(&self, actor: &Actor) -> &Image {
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
        let pos = world_to_screen_coords(actor.pos);
        let params = DrawParam::new()
            .src(Rect::new(tile_offs, 0.0, tile_offs + sprite_wid, 1.0))
            .dest(pos);

        canvas.draw(self.actor_image(actor), params)
    }
}

/// Translates the world coordinate system, which has Y pointing up
/// to the screen coordinate system, which has Y pointing downward.
fn world_to_screen_coords(point: Point2) -> Point2 {
    let x = point.x;
    let y = SCREEN_HEIGHT - point.y;
    Point2::new(x.round(), y.round())
}
