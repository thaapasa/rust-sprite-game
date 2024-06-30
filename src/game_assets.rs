use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Image, Rect};

use crate::actor::{Actor, ActorType};

pub struct GameAssets {
    pub background: Image,
    pub player_idle_tiles: Image,
    pub ground_tiles: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let background = Image::from_path(ctx, "/background.png")?;
        let player_idle_tiles = Image::from_path(ctx, "/idle-tileset.png")?;
        let ground_tiles = Image::from_path(ctx, "/background-tileset.png")?;
        Ok(GameAssets {
            background,
            player_idle_tiles,
            ground_tiles,
        })
    }

    pub fn actor_image(&self, actor: &Actor) -> &Image {
        match actor.tag {
            ActorType::Player => &self.player_idle_tiles,
            ActorType::GroundBlock { x, y } => &self.ground_tiles,
        }
    }

    pub fn draw_actor(
        &self,
        actor: &Actor,
        canvas: &mut graphics::Canvas,
        offset: u8,
        sprite_count: u8,
        scale: Vec2,
    ) {
        let sprite_wid = 1.0 / sprite_count as f32;
        let tile_offs = offset as f32 * sprite_wid;
        let pos = actor.screen_coords(&scale);
        let params = DrawParam::new()
            .src(Rect::new(tile_offs, 0.0, tile_offs + sprite_wid, 1.0))
            .dest(pos)
            .scale(scale);

        canvas.draw(self.actor_image(actor), params)
    }
}
