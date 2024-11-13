use std::hash::{Hash, Hasher};

use ggmath::prelude::*;

use super::tile::Tile;

/// The default number of noise samples to perform. Higher values result in more detailed noise
/// at the cost of performance.
pub const NOISE_DEFAULT_LEVELS: usize = 4;
/// The default scale of the noise. Higher values result in larger features.
/// Higher values also require higher `NOISE_LEVELS` to maintain detail.
pub const NOISE_DEFAULT_SCALE: f64 = 4.0;
/// The default smoothness of the noise.
pub const NOISE_DEFAULT_SMOOTHNESS: f64 = 1.0;
/// The default detail strength of the noise. Higher values result in stronger fine details.
pub const NOISE_DETAIL_STRENGTH: f64 = 0.1;

/// The maximum elevation of the world.
/// This is the highest elevation that can be sampled from the elevation noise, in tiles.
pub const MAX_ELEVATION: isize = 20;
/// The minimum elevation of the world.
/// This is the lowest elevation that can be sampled from the elevation noise, in tiles.
pub const MIN_ELEVATION: isize = -15;
/// The water level of the world.
/// Terrain with an elevation below this level will be submerged in water.
pub const WATER_LEVEL: isize = 0;

/// The temperature at which water freezes.
pub const FREEZING_TEMPERATURE: f32 = 0.3;

/// Used to generate the game world.
pub struct WorldGenerator {
    temperature_noise: Noise<2>,
    humidity_noise: Noise<2>,
    elevation_noise: Noise<2>,
}

impl WorldGenerator {
    /// Create a new world generator with the given seed.
    pub fn new(seed: u64) -> Self {
        // Create noise generators
        let temperature_noise = Noise::new(
            seed,
            NOISE_DEFAULT_LEVELS,
            NOISE_DEFAULT_SCALE,
            NOISE_DEFAULT_SMOOTHNESS,
            NOISE_DETAIL_STRENGTH,
        );
        let humidity_noise = Noise::new(
            seed ^ 12345,
            NOISE_DEFAULT_LEVELS,
            NOISE_DEFAULT_SCALE,
            NOISE_DEFAULT_SMOOTHNESS,
            NOISE_DETAIL_STRENGTH,
        );
        let elevation_noise = Noise::new(
            seed ^ 23456,
            NOISE_DEFAULT_LEVELS,
            NOISE_DEFAULT_SCALE,
            NOISE_DEFAULT_SMOOTHNESS,
            NOISE_DETAIL_STRENGTH,
        );

        Self {
            temperature_noise,
            humidity_noise,
            elevation_noise,
        }
    }

    /// Sample the climate at the given XZ position.
    pub fn climate_at(&self, position: Vector2<isize>) -> GenClimate {
        let position = position.convert_to().unwrap();
        let temperature = self
            .temperature_noise
            .sample_f64(position + vector!(1234.0, 0.0)) as f32;
        let humidity = self
            .humidity_noise
            .sample_f64(position + vector!(3456.0, 0.0)) as f32;
        GenClimate {
            temperature,
            humidity,
        }
    }

    /// Sample the elevation of the terrain surface at the given XZ position.
    /// Returns a value between 0.0 and 1.0.
    /// Higher values indicate higher elevation.
    pub fn elevation_at(&self, position: Vector2<isize>) -> isize {
        let elevation_noise = self
            .elevation_noise
            .sample_f64(position.convert_to().unwrap() + vector!(5678.0, 0.0));
        MIN_ELEVATION + (elevation_noise * (MAX_ELEVATION - MIN_ELEVATION) as f64) as isize
    }

    /// Check how far below the surface & water level the given tile position is.
    pub fn depth_at(&self, position: Vector3<isize>) -> GenDepth {
        // Get the elevation at the XZ position.
        let elevation = self.elevation_at(position.xz());

        // Calculate the surface depth
        let surface_depth = elevation - position.y();

        // Calculate the water depth
        let water_depth = WATER_LEVEL - position.y();

        GenDepth {
            surface_depth,
            water_depth,
        }
    }

    /// Check how far below the water level the given position is.
    /// If this number is negative, then the position is above the water level.
    pub fn water_depth_at(&self, position: Vector3<isize>) -> isize {
        WATER_LEVEL - position.y()
    }

    /// Sample the tile at the given XYZ world position.
    pub fn sample_tile(&self, position: Vector3<isize>) -> Option<Tile> {
        // Create a TileRng for the tile
        let rng = TileRng::new(position);

        // Sample the depth and climate at the position
        let climate = self.climate_at(position.xz());
        let depth = self.depth_at(position);

        // Generate the tile
        Tile::from_samples(rng, climate, depth)
    }
}

/// Represents the type of terrain at a position in the world during generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    Air,
    Surface,
    Underground,
    Water,
}

/// Represents the depth of a position in the world during generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenDepth {
    surface_depth: isize,
    water_depth: isize,
}

impl GenDepth {
    /// Get the surface depth.
    /// This is the distance below the surface of the terrain in tiles.
    pub fn surface(&self) -> isize {
        self.surface_depth
    }

    /// Get the water depth.
    /// This is the distance below the water level in tiles.
    pub fn water(&self) -> isize {
        self.water_depth
    }

    /// Check if the given position is at or under the terrain surface.
    pub fn is_in_ground(&self) -> bool {
        self.surface_depth >= 0
    }

    /// Check if the given position is under the water level and not under the surface.
    /// This is useful for checking if a position is submerged in water during generation.
    pub fn is_in_water(&self) -> bool {
        self.water_depth >= 0 && !self.is_in_ground()
    }

    /// Check if the tile is at the surface.
    pub fn is_surface(&self) -> bool {
        self.surface_depth == 0
    }

    /// Get the type of terrain at the given position.
    pub fn terrain_type(&self) -> TerrainType {
        if self.is_in_water() {
            TerrainType::Water
        } else if self.is_in_ground() {
            if self.is_surface() {
                TerrainType::Surface
            } else {
                TerrainType::Underground
            }
        } else {
            TerrainType::Air
        }
    }
}

/// Represents a base biome during world generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseBiome {
    Grassland,
    Desert,
    Tundra,
}

/// Represents the climate of a position in the world.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenClimate {
    temperature: f32,
    humidity: f32,
}

impl GenClimate {
    /// Get the temperature of the climate.
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Get the humidity of the climate.
    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    /// Get the base biome of the climate.
    pub fn base_biome(&self) -> BaseBiome {
        if self.temperature < FREEZING_TEMPERATURE {
            BaseBiome::Tundra
        } else if self.humidity < 0.4 && self.temperature > 0.6 {
            BaseBiome::Desert
        } else {
            BaseBiome::Grassland
        }
    }
}

/// Used to randomize features of a tile.
#[derive(Debug, Clone)]
pub struct TileRng {
    lcg: Lcg,
}

impl TileRng {
    /// Create a new tile RNG using the hash of the given value as a seed.
    pub fn new(world_tile_position: Vector3<isize>) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        world_tile_position.hash(&mut hasher);
        let seed = hasher.finish();
        Self {
            lcg: Lcg::new(seed),
        }
    }

    /// Returns true with the given probability.
    pub fn one_in<T: OneIn>(&mut self, probability: T) -> bool {
        self.lcg.next::<T>().__one_in(probability)
    }
}

/// Trait for TileRng::one_in roll results
pub trait OneIn: FromRandom {
    /// Check the roll result against the given chance.
    fn __one_in(&self, probability: Self) -> bool;
}

impl OneIn for f32 {
    fn __one_in(&self, probability: Self) -> bool {
        (self * probability) < 1.0
    }
}

impl OneIn for u32 {
    fn __one_in(&self, probability: Self) -> bool {
        self % probability == 0
    }
}
