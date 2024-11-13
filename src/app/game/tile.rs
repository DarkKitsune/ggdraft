use ggmath::prelude::*;

use crate::color;

use super::world_generator::{BaseBiome, GenClimate, GenDepth, TerrainType, TileRng};

/// Represents the type of a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Dirt,
    Grass,
    Water,
    Sand,
    Rock,
    Snow,
    Ice,
}

impl TileType {
    pub fn color(&self) -> Vector4<f32> {
        match self {
            TileType::Dirt => color::BROWN.lerp(&color::BLACK, 0.2),
            TileType::Grass => color::LIME.lerp(&color::BLACK, 0.2),
            TileType::Water => color::BLUE,
            TileType::Sand => color::YELLOW.lerp(&color::ORANGE, 0.2),
            TileType::Rock => color::GRAY,
            TileType::Snow => color::WHITE,
            TileType::Ice => color::CYAN,
        }
    }
}

/// A tile in the world.
#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    /// Create a new tile.
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }

    /// Get the type of tile this is.
    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    /// Generate a new tile based on the given noise values.
    pub fn from_samples(mut rng: TileRng, climate: GenClimate, depth: GenDepth) -> Option<Self> {
        let terrain_type = depth.terrain_type();
        let base_biome = climate.base_biome();

        match terrain_type {
            TerrainType::Air => None,
            TerrainType::Water => match base_biome {
                BaseBiome::Tundra => Some(Self::new(TileType::Ice)),
                _ => Some(Self::new(TileType::Water)),
            },
            TerrainType::Surface => match base_biome {
                BaseBiome::Grassland => Some(Self::new(TileType::Grass)),
                BaseBiome::Desert => Some(Self::new(TileType::Sand)),
                BaseBiome::Tundra => Some(Self::new(TileType::Snow)),
            },
            TerrainType::Underground => {
                // Create a rock tile with a 1 in 4 chance.
                if rng.one_in(4) {
                    Some(Self::new(TileType::Rock))
                } else {
                    Some(Self::new(TileType::Dirt))
                }
            }
        }
    }

    /// Get the color of the tile.
    pub fn color(&self) -> Vector4<f32> {
        self.tile_type.color()
    }

    /// Generate vertices for the tile.
    pub fn generate_vertices(
        &self,
        chunk_world_position: Vector3<f32>,
        position_in_chunk: Vector3<usize>,
        visible_from: &TileVisibility,
        positions: &mut Vec<Vector3<f32>>,
        normals: &mut Vec<Vector3<f32>>,
        colors: &mut Vec<Vector4<f32>>,
        indices: &mut Vec<u32>,
        current_index: &mut u32,
    ) {
        let base_position = chunk_world_position + position_in_chunk.convert_to::<f32>().unwrap();
        let color = self.color();

        // Negative X face.
        if visible_from.negative_x {
            // Push the vertex positions for the negative X face.
            positions.push(base_position + vector!(0.0, 0.0, 0.0));
            positions.push(base_position + vector!(0.0, 0.0, 1.0));
            positions.push(base_position + vector!(0.0, 1.0, 1.0));
            positions.push(base_position + vector!(0.0, 1.0, 0.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(-1.0, 0.0, 0.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }

        // Positive X face.
        if visible_from.positive_x {
            // Push the vertex positions for the positive X face.
            positions.push(base_position + vector!(1.0, 0.0, 0.0));
            positions.push(base_position + vector!(1.0, 1.0, 0.0));
            positions.push(base_position + vector!(1.0, 1.0, 1.0));
            positions.push(base_position + vector!(1.0, 0.0, 1.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(1.0, 0.0, 0.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }

        // Negative Y face.
        if visible_from.negative_y {
            // Push the vertex positions for the negative Y face.
            positions.push(base_position + vector!(0.0, 0.0, 0.0));
            positions.push(base_position + vector!(1.0, 0.0, 0.0));
            positions.push(base_position + vector!(1.0, 0.0, 1.0));
            positions.push(base_position + vector!(0.0, 0.0, 1.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(0.0, -1.0, 0.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }

        // Positive Y face.
        if visible_from.positive_y {
            // Push the vertex positions for the positive Y face.
            positions.push(base_position + vector!(0.0, 1.0, 0.0));
            positions.push(base_position + vector!(0.0, 1.0, 1.0));
            positions.push(base_position + vector!(1.0, 1.0, 1.0));
            positions.push(base_position + vector!(1.0, 1.0, 0.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(0.0, 1.0, 0.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }

        // Negative Z face.
        if visible_from.negative_z {
            // Push the vertex positions for the negative Z face.
            positions.push(base_position + vector!(0.0, 0.0, 0.0));
            positions.push(base_position + vector!(0.0, 1.0, 0.0));
            positions.push(base_position + vector!(1.0, 1.0, 0.0));
            positions.push(base_position + vector!(1.0, 0.0, 0.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(0.0, 0.0, -1.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }

        // Positive Z face.
        if visible_from.positive_z {
            // Push the vertex positions for the positive Z face.
            positions.push(base_position + vector!(0.0, 0.0, 1.0));
            positions.push(base_position + vector!(1.0, 0.0, 1.0));
            positions.push(base_position + vector!(1.0, 1.0, 1.0));
            positions.push(base_position + vector!(0.0, 1.0, 1.0));

            // Push normals and colors
            for _ in 0..4 {
                normals.push(vector!(0.0, 0.0, 1.0));
                colors.push(color);
            }

            // Push indices
            indices.push(*current_index);
            indices.push(*current_index + 1);
            indices.push(*current_index + 2);
            indices.push(*current_index + 2);
            indices.push(*current_index + 3);
            indices.push(*current_index);

            *current_index += 4;
        }
    }
}

/// Represents the sides which a tile is visible from.
pub struct TileVisibility {
    pub negative_x: bool,
    pub positive_x: bool,
    pub negative_y: bool,
    pub positive_y: bool,
    pub negative_z: bool,
    pub positive_z: bool,
}

impl TileVisibility {
    /// Create a new `TileVisibility`.
    pub fn new(
        negative_x: bool,
        positive_x: bool,
        negative_y: bool,
        positive_y: bool,
        negative_z: bool,
        positive_z: bool,
    ) -> Self {
        Self {
            negative_x,
            positive_x,
            negative_y,
            positive_y,
            negative_z,
            positive_z,
        }
    }
}
