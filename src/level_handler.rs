use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use ggez::GameResult;
use ggez::graphics::Rect;

use crate::actor::Actor;
use crate::constants::{GROUND_TILE_HEIGHT, GROUND_TILE_WIDTH};
use crate::primitives::RectExt;

pub struct LevelHandler {
    pub actors: Vec<Actor>,
    bbox: Rect,
}

impl LevelHandler {
    pub fn new(file: &str, width: usize, height: usize) -> GameResult<LevelHandler> {
        let tiles = LevelTiles::new();
        let level = LevelBuilder::load_level(file, &tiles, width, height)?;
        let actors = LevelBuilder::create_actors(&level);
        return Ok(LevelHandler {
            actors,
            bbox: Rect {
                x: 0.0,
                y: 0.0,
                w: width as f32 * GROUND_TILE_WIDTH,
                h: height as f32 * GROUND_TILE_HEIGHT,
            },
        });
    }

    pub fn get_collisions(&self, bbox: &Rect) -> Vec<&Actor> {
        if !self.bbox.collides_with(&bbox) {
            return Vec::new();
        }

        return self
            .actors
            .iter()
            .filter(|a| a.bbox.collides_with(bbox))
            .collect();
    }

    pub fn collides_with(&self, bbox: &Rect) -> bool {
        if !self.bbox.collides_with(&bbox) {
            return false;
        }
        for a in &self.actors {
            if a.bbox.collides_with(bbox) {
                return true;
            }
        }
        return false;
    }
}

struct LevelBuilder {}

impl LevelBuilder {
    fn load_level(
        file: &str,
        tiles: &LevelTiles,
        width: usize,
        height: usize,
    ) -> GameResult<Vec<Vec<TileType>>> {
        let path = Path::new(file);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let level = reader
            .lines()
            .take(height as usize)
            .map(|r| r.unwrap_or(String::from("")))
            .map(|r| Self::read_row(tiles, &r, width))
            .collect();

        return Ok(level);
    }

    fn create_actors(level: &Vec<Vec<TileType>>) -> Vec<Actor> {
        let height = level.len();
        let mut actors = Vec::new();
        for (y, row) in level.into_iter().enumerate() {
            for (x, tile) in row.into_iter().enumerate() {
                match LevelBuilder::create_tile(tile, x, height - y - 1) {
                    Some(actor) => actors.push(actor),
                    _ => (),
                }
            }
        }
        return actors;
    }

    fn create_tile(tile: &TileType, x: usize, y: usize) -> Option<Actor> {
        match tile.char {
            ' ' => None,
            _ => Some(Actor::create_ground(tile, x, y)),
        }
    }

    fn read_row(tiles: &LevelTiles, line: &str, width: usize) -> Vec<TileType> {
        let mut row: Vec<TileType> = line
            .chars()
            .take(width) // Ensure we only process up to `width` characters
            .map(|c| tiles.for_char(c))
            .collect();

        // If the length of the row is less than width, append empty tiles
        while row.len() < width {
            row.push(tiles.empty);
        }

        row
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TileType {
    name: &'static str,
    char: char,
    /// Tile x index in the tile set; from upper left corner, starting at 0.
    pub x: usize,
    /// Tile y index in the tile set; from upper left corner, starting at 0.
    pub y: usize,
}

impl TileType {
    fn new(name: &'static str, char: char, x: usize, y: usize) -> Self {
        TileType { name, char, x, y }
    }
}

pub struct LevelTiles {
    empty: TileType,
    tile_map: HashMap<char, TileType>,
}

impl LevelTiles {
    pub fn new() -> Self {
        let empty = TileType::new("EMPTY", ' ', 0, 0);
        let tile_types = vec![
            empty,
            TileType::new("GROUND", '#', 11, 1),
            TileType::new("TOP", '^', 1, 0),
            TileType::new("BOTTOM", 'v', 8, 4),
            TileType::new("LEFT", '<', 0, 1),
            TileType::new("RIGHT", '>', 3, 1),
            TileType::new("LEFT_RIGHT", 'H', 4, 1),
            TileType::new("TOP_LEFT", '┌', 0, 0),
            TileType::new("TOP_RIGHT", '┐', 2, 0),
            TileType::new("TOP_BOTTOM", '=', 1, 4),
            TileType::new("BOTTOM_LEFT", '└', 7, 4),
            TileType::new("BOTTOM_RIGHT", '┘', 9, 4),
            TileType::new("TOP_BOTTOM_LEFT", '├', 0, 4),
            TileType::new("TOP_BOTTOM_RIGHT", '┤', 2, 4),
            TileType::new("TOP_RIGHT_LEFT", '┬', 4, 0),
            TileType::new("BOTTOM_RIGHT_LEFT", '┴', 8, 4),
            TileType::new("ALL_BORDERS", '┼', 5, 2),
        ];

        let tile_map: HashMap<char, TileType> = tile_types
            .into_iter()
            .map(|tile| (tile.char, tile))
            .collect();
        return LevelTiles { empty, tile_map };
    }

    pub fn for_char(&self, char: char) -> TileType {
        let tile = self.tile_map.get(&char).expect("Unknown tile type");
        return *tile;
    }
}
