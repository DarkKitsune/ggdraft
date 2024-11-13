use std::rc::Rc;

use anyhow::{anyhow, Result};
use ggmath::{init_array, prelude::*};
use ggutil::prelude::MaybeOwned;

use crate::gfx::{
    vertex_layout::VertexLayout,
    vertex_list::{IntoVertexList, VertexList, VertexListInput},
};

use super::{
    tile::{Tile, TileVisibility},
    world::{ChunkSpaceConversion, WorldSpaceConversion},
    world_generator::WorldGenerator,
};

/// The size of a chunk in the world.
/// This is the number of tiles in each dimension of a chunk.
pub const CHUNK_SIZE: usize = 32;
/// The volume of a chunk in the world.
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// The step size in a chunk's tile array for each increment of Y.
const CHUNK_STEP_Y: usize = CHUNK_SIZE;
/// The step size in a chunk's tile array for each increment of Z.
const CHUNK_STEP_Z: usize = CHUNK_SIZE * CHUNK_SIZE;

/// A chunk in the world.
pub struct Chunk {
    /// The coordinates of the chunk in the world.
    coord: Vector3<isize>,
    /// The tiles in the chunk.
    tiles: [Option<Tile>; CHUNK_VOLUME],
    /// Whether the chunk is empty.
    empty: bool,
}

impl Chunk {
    /// Generate a new chunk using the given `WorldGenerator`.
    pub fn generate(coord: Vector3<isize>, generator: &WorldGenerator) -> Self {
        // Initialize the array with tiles sampled from the generator.
        let mut empty = true;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let tiles = init_array!(
            [Option<Tile>; CHUNK_VOLUME],
            mut |_| {
                // Sample the tile at the current position from the generator.
                let tile = generator.sample_tile(vector!(x, y, z).chunk_to_world(coord));

                // If the tile is not empty, then the chunk is not empty.
                if tile.is_some() {
                    empty = false;
                }

                // Increment the position.
                x += 1;
                if x >= CHUNK_SIZE as isize {
                    x = 0;
                    y += 1;
                    if y >= CHUNK_SIZE as isize {
                        y = 0;
                        z += 1;
                    }
                }

                tile
            }
        );

        Self { coord, tiles, empty }
    }

    /// Convert an in-chunk tile position to an index in the chunk's tile array.
    fn pos_to_index(in_chunk_position: Vector3<usize>) -> usize {
        in_chunk_position.x()
            + in_chunk_position.y() * CHUNK_STEP_Y
            + in_chunk_position.z() * CHUNK_STEP_Z
    }

    /// Get the tile at the given in-chunk tile position.
    /// Returns None if the tile is empty or the position is out of bounds.
    pub fn get_tile(&self, in_chunk_position: Vector3<usize>) -> Option<&Tile> {
        // Calculate the index of the tile in the chunk's tile array.
        let idx = Self::pos_to_index(in_chunk_position);

        // Get the tile at the index.
        self.tiles.get(idx)?.as_ref()
    }

    /// Get the mutable tile at the given in-chunk tile position.
    /// Returns None if the tile is empty or the position is out of bounds.
    pub fn get_tile_mut(&mut self, in_chunk_position: Vector3<usize>) -> Option<&mut Tile> {
        // Calculate the index of the tile in the chunk's tile array.
        let idx = Self::pos_to_index(in_chunk_position);

        // Get the tile at the index.
        self.tiles.get_mut(idx)?.as_mut()
    }

    /// Set the tile at the given in-chunk tile position.
    /// Returns an error if the position is out of bounds.
    pub fn set_tile(&mut self, in_chunk_position: Vector3<usize>, tile: Tile) -> Result<()> {
        // Calculate the index of the tile in the chunk's tile array.
        let idx = in_chunk_position.x()
            + in_chunk_position.y() * CHUNK_STEP_Y
            + in_chunk_position.z() * CHUNK_STEP_Z;

        // Replace the tile at the index in the chunk's tile array if it exists, otherwise return an error.
        self.tiles
            .get_mut(idx)
            .ok_or_else(|| anyhow!("Position out of bounds: {:?}", in_chunk_position))?
            .replace(tile);
        Ok(())
    }

    /// Get the coordinates of the chunk in the world.
    pub const fn coordinates(&self) -> Vector3<isize> {
        self.coord
    }

    /// Check if the chunk is empty.
    pub const fn is_empty(&self) -> bool {
        self.empty
    }

    /// Convert a world-space position to a position local to this chunk.
    /// Returns None if the position is out of bounds of the chunk.
    pub fn world_to_local(&self, world_position: Vector3<f32>) -> Option<Vector3<f32>> {
        // Calculate the position of the tile in the chunk.
        let in_chunk_position = world_position.world_to_chunk();
        // Check if the position is out of bounds.
        if in_chunk_position.x() >= CHUNK_SIZE as f32
            || in_chunk_position.y() >= CHUNK_SIZE as f32
            || in_chunk_position.z() >= CHUNK_SIZE as f32
        {
            None
        } else {
            Some(in_chunk_position)
        }
    }

    /// Convert a local position to a world-space position.
    pub fn local_to_world(&self, local_position: Vector3<f32>) -> Vector3<f32> {
        // Calculate the position of the tile in the world.
        local_position.chunk_to_world(self.coord)
    }

    /// Generate a `VertexList` for rendering the chunk.
    pub fn to_vertices(&self, layout: Rc<VertexLayout>) -> Result<VertexList> {
        // Exit early with an error if the chunk is empty.
        if self.is_empty() {
            return Err(anyhow!("Chunk is empty"));
        }

        let chunk_world_position = self.coord.chunk_coord_to_world();

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();

        let mut current_index = 0;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        // Keeps track of whether the previous tile was None, since it is always
        // in the negative X direction unless we have stepped in the Y direction.
        let mut nx_none = true;

        // Iterate over every tile in the chunk, generating vertices and indices.
        for (tile_idx, tile) in self.tiles.iter().enumerate() {
            // Only output vertices and indices if the tile is not None.
            if let Some(tile) = tile {
                // First check which faces are visible by checking the surrounding tiles.
                // We already know whether the negative X direction is None.
                // Also short circuit the check if we are at the edge of the chunk.
                // Check the tile in the positive X direction.
                let px_none =
                    x == CHUNK_SIZE - 1 || self.tiles.get(tile_idx + 1).unwrap_or(&None).is_none();
                // Check the tile in the negative Y direction.
                let ny_none = y == 0
                    || self
                        .tiles
                        .get(tile_idx - CHUNK_SIZE)
                        .unwrap_or(&None)
                        .is_none();
                // Check the tile in the positive Y direction.
                let py_none = y == CHUNK_SIZE - 1
                    || self
                        .tiles
                        .get(tile_idx + CHUNK_SIZE)
                        .unwrap_or(&None)
                        .is_none();
                // Check the tile in the negative Z direction.
                let nz_none = z == 0
                    || self
                        .tiles
                        .get(tile_idx - CHUNK_STEP_Z)
                        .unwrap_or(&None)
                        .is_none();
                // Check the tile in the positive Z direction.
                let pz_none = z == CHUNK_SIZE - 1
                    || self
                        .tiles
                        .get(tile_idx + CHUNK_STEP_Z)
                        .unwrap_or(&None)
                        .is_none();
                // Create the TileVisibility object.
                let tile_visibility =
                    TileVisibility::new(nx_none, px_none, ny_none, py_none, nz_none, pz_none);

                tile.generate_vertices(
                    chunk_world_position,
                    vector!(x, y, z),
                    &tile_visibility,
                    &mut positions,
                    &mut normals,
                    &mut colors,
                    &mut indices,
                    &mut current_index,
                );
            }

            // Increment the position.
            x += 1;
            if x >= CHUNK_SIZE {
                x = 0;
                y += 1;
                if y >= CHUNK_SIZE {
                    y = 0;
                    z += 1;
                }

                // If we have stepped in the Y direction, then reset nx_none.
                nx_none = true;
            } else {
                // If we have not stepped in the Y direction, then update nx_none.
                nx_none = tile.is_none();
            }
        }

        VertexList::new(
            layout,
            &[
                VertexListInput::Position(&positions),
                VertexListInput::Normal(&normals),
                VertexListInput::Color(&colors),
            ],
            indices,
        )
    }
}

impl<'a> IntoVertexList<'a> for &Chunk {
    fn into_vertex_list(self, layout: Rc<VertexLayout>) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Owned(self.to_vertices(layout).unwrap_or_else(|e| {
            panic!("Failed to generate vertices for chunk: {}", e);
        }))
    }
}

impl<'a> IntoVertexList<'a> for &mut Chunk {
    fn into_vertex_list(self, layout: Rc<VertexLayout>) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Owned(self.to_vertices(layout).unwrap_or_else(|e| {
            panic!("Failed to generate vertices for chunk: {}", e);
        }))
    }
}