use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use ggez::GameResult;

pub struct LevelHandler {
    width: u8,
    height: u8,
    level: Vec<Vec<TileType>>,
}

impl LevelHandler {
    pub fn new(file: &str, width: u8, height: u8) -> GameResult<LevelHandler> {
        let tiles = LevelTiles::new();
        let level = Self::load_level(file, &tiles, width, height)?;
        return Ok(LevelHandler {
            width,
            height,
            level,
        });
    }

    fn load_level(
        file: &str,
        tiles: &LevelTiles,
        width: u8,
        height: u8,
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

    fn read_row(tiles: &LevelTiles, line: &str, width: u8) -> Vec<TileType> {
        let mut row: Vec<TileType> = line
            .chars()
            .take(width as usize) // Ensure we only process up to `width` characters
            .map(|c| tiles.for_char(c))
            .collect();

        // If the length of the row is less than width, append empty tiles
        while row.len() < width as usize {
            row.push(tiles.empty);
        }

        row
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TileType {
    name: &'static str,
    char: char,
    x: i32,
    y: i32,
}

impl TileType {
    fn new(name: &'static str, char: char, x: i32, y: i32) -> Self {
        TileType { name, char, x, y }
    }
}

pub struct LevelTiles {
    pub empty: TileType,
    tile_map: HashMap<char, TileType>,
}

impl LevelTiles {
    pub fn new() -> Self {
        let empty = TileType::new("EMPTY", ' ', -1, -1);
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
