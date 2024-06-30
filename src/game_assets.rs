use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Image};

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
        frame_offset: usize,
        scale: Vec2,
    ) {
        let img = self.actor_image(actor);
        let src = actor.tile_offset(&img, frame_offset);
        let dest = actor.screen_coords(&scale);
        let params = DrawParam::new().src(src).dest(dest).scale(scale);

        canvas.draw(img, params)
    }
}
