use ggmath::prelude::*;

use crate::color;

/// A tile in the world
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    /// Create a new tile
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }

    /// Get the type of tile this is
    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }
}

/// Represents the type of a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Water,
    Sand,
    Rock,
}

impl TileType {
    pub fn color(&self) -> Vector4<f32> {
        match self {
            TileType::Grass => color::LIME.lerp(&color::BLACK, 0.2),
            TileType::Water => color::BLUE,
            TileType::Sand => color::YELLOW.lerp(&color::ORANGE, 0.2),
            TileType::Rock => color::GRAY,
        }
    }
}
