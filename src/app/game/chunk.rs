use anyhow::{anyhow, Result};
use ggmath::{init_array, prelude::*};

use super::tile::Tile;

/// The size of a chunk in the world.
/// This is the number of tiles in each dimension of a chunk.
pub const CHUNK_SIZE: usize = 16;

/// The volume of a chunk in the world.
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// The step size in a chunk's tile array for each increment of Y.
const CHUNK_STEP_Y: usize = CHUNK_SIZE;

/// The step size in a chunk's tile array for each increment of Z.
const CHUNK_STEP_Z: usize = CHUNK_SIZE * CHUNK_SIZE;

/// A chunk in the world.
pub struct Chunk {
    tiles: [Option<Tile>; CHUNK_VOLUME],
}

impl Chunk {
    /// Create a new chunk
    pub const fn new() -> Self {
        Self {
            tiles: Self::__init_none_array(),
        }
    }

    // Helper functions for generating a const array of None
    const fn __init_none_array() -> [Option<Tile>; CHUNK_VOLUME] {
        init_array!([Option<Tile>; CHUNK_VOLUME], (), const Self::__init_none_array_idx)
    }

    const fn __init_none_array_idx(_idx: usize) -> Option<Tile> {
        None
    }

    // Create a new chunk with the given tiles
    pub const fn with_tiles(tiles: [Option<Tile>; CHUNK_VOLUME]) -> Self {
        Self { tiles }
    }

    /// Create a new chunk with tiles returned by the given function
    pub fn with_tiles_fn<F>(f: impl Fn(usize, usize, usize) -> Tile) -> Self {
        // Start with an array of None
        let mut tiles = Self::__init_none_array();

        // Fill the array with the tiles returned by the function
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    tiles[x + y * CHUNK_STEP_Y + z * CHUNK_STEP_Z] = Some(f(x, y, z));
                }
            }
        }

        Self { tiles }
    }

    /// Get the tile at the given in-chunk position
    /// Returns None if the tile is empty or the position is out of bounds
    pub fn get_tile(&self, in_chunk_position: Vector3<usize>) -> Option<&Tile> {
        // Calculate the index of the tile in the chunk's tile array
        let idx = in_chunk_position.x()
            + in_chunk_position.y() * CHUNK_STEP_Y
            + in_chunk_position.z() * CHUNK_STEP_Z;

        // Get the tile at the index
        self.tiles.get(idx)?.as_ref()
    }

    /// Get the mutable tile at the given in-chunk position
    /// Returns None if the tile is empty or the position is out of bounds
    pub fn get_tile_mut(&mut self, in_chunk_position: Vector3<usize>) -> Option<&mut Tile> {
        // Calculate the index of the tile in the chunk's tile array
        let idx = in_chunk_position.x()
            + in_chunk_position.y() * CHUNK_STEP_Y
            + in_chunk_position.z() * CHUNK_STEP_Z;

        // Get the tile at the index
        self.tiles.get_mut(idx)?.as_mut()
    }

    /// Set the tile at the given in-chunk position
    /// Returns an error if the position is out of bounds
    pub fn set_tile(&mut self, in_chunk_position: Vector3<usize>, tile: Tile) -> Result<()> {
        // Calculate the index of the tile in the chunk's tile array
        let idx = in_chunk_position.x()
            + in_chunk_position.y() * CHUNK_STEP_Y
            + in_chunk_position.z() * CHUNK_STEP_Z;

        // Replace the tile at the index in the chunk's tile array if it exists, otherwise return an error
        self.tiles
            .get_mut(idx)
            .ok_or_else(|| anyhow!("Position out of bounds: {:?}", in_chunk_position))?
            .replace(tile);
        Ok(())
    }
}
