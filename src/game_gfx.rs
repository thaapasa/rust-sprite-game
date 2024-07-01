use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};

use crate::actor::{Actor, ActorType};
use crate::constants::SCREEN_HEIGHT;
use crate::game::SpriteGame;
use crate::game_assets::GameAssets;
use crate::primitives::{Direction, Point2};

pub struct GraphicsHandler {
    pub assets: GameAssets,
    player_bbox: Mesh,
    ground_bbox: Mesh,
}

impl GraphicsHandler {
    pub fn new(ctx: &mut Context) -> GameResult<GraphicsHandler> {
        let assets = GameAssets::new(ctx).expect("Could not initialize Game Assets");
        let player_bbox = Self::create_bbox(ctx, 42.0, 74.0, Color::GREEN)?;
        let ground_bbox = Self::create_bbox(ctx, 32.0, 32.0, Color::BLUE)?;
        return Ok(GraphicsHandler {
            assets,
            player_bbox,
            ground_bbox,
        });
    }

    fn create_bbox(ctx: &mut Context, width: f32, height: f32, color: Color) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        let ratio = (width + height) / 2.0;
        let bbox_data = mb
            .rectangle(
                DrawMode::stroke(1.0 / ratio),
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 1.0,
                    h: 1.0,
                },
                color,
            )?
            .build();
        return Ok(Mesh::from_data(ctx, bbox_data));
    }

    pub fn get_screen_coords(src: &Rect, scale: &Vec2) -> Rect {
        let y = SCREEN_HEIGHT - src.y - src.h;
        return Rect {
            x: src.x * scale.x,
            y: y * scale.y,
            w: src.w * scale.x,
            h: src.h * scale.y,
        };
    }

    pub fn draw_actor(&self, actor: &Actor, canvas: &mut Canvas, game: &SpriteGame, scale: Vec2) {
        let img = self.assets.actor_image(actor, game);
        let src = actor.tile_offset(&img, game);
        let dest = Self::get_screen_coords(&actor.draw_rect(), &scale);
        let params = DrawParam::new().src(src).dest(dest.point());

        let facing = match actor.facing {
            Direction::Left => params
                .scale(Vec2::new(-scale.x, scale.y))
                .offset(Point2::new(1.0, 0.0)),
            _ => params.scale(scale),
        };

        canvas.draw(img, facing)
    }

    pub fn draw_bbox(&self, actor: &Actor, canvas: &mut Canvas, scale: Vec2) {
        let rect = Self::get_screen_coords(&actor.bbox_rect(), &scale);
        let bbox = match actor.tag {
            ActorType::Player => &self.player_bbox,
            ActorType::GroundBlock { x, y } => &self.ground_bbox,
        };
        canvas.draw(bbox, DrawParam::new().dest(rect.point()).scale(rect.size()));
    }
}
