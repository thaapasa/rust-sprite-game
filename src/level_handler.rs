use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use ggez::GameResult;

use crate::actor::Actor;

pub struct LevelHandler {
    width: usize,
    height: usize,
    level: Vec<Vec<TileType>>,
    actors: Vec<Vec<Option<Actor>>>,
}

impl LevelHandler {
    pub fn new(file: &str, width: usize, height: usize) -> GameResult<LevelHandler> {
        let tiles = LevelTiles::new();
        let level = LevelBuilder::load_level(file, &tiles, width, height)?;
        let actors = LevelBuilder::create_actors(&level);
        return Ok(LevelHandler {
            width,
            height,
            level,
            actors,
        });
    }

    pub fn traverse_actors<F>(&self, callback: &mut F)
    where
        F: FnMut(&Actor),
    {
        for row in self.actors.iter() {
            for item in row.iter() {
                match item {
                    Some(a) => callback(&a),
                    _ => (),
                }
            }
        }
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

    fn create_actors(level: &Vec<Vec<TileType>>) -> Vec<Vec<Option<Actor>>> {
        return level
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, tile)| LevelBuilder::create_actor(tile, x, level.len() - y))
                    .collect()
            })
            .collect();
    }

    fn create_actor(tile: &TileType, x: usize, y: usize) -> Option<Actor> {
        match tile.char {
            ' ' => None,
            _ => Some(Actor::create_tile(tile, x as f32 * 32.0, y as f32 * 32.0)),
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
    pub x: usize,
    pub y: usize,
}

impl TileType {
    fn new(name: &'static str, char: char, x: usize, y: usize) -> Self {
        TileType { name, char, x, y }
    }
}

pub struct LevelTiles {
    pub empty: TileType,
    tile_map: HashMap<char, TileType>,
}

impl LevelTiles {
    pub fn new() -> Self {
        let empty = TileType::new("EMPTY", ' ', 0, 0);
        let tile_types = vec![
            empty,
            TileType::new("GROUND", '#', 5, 5),
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
