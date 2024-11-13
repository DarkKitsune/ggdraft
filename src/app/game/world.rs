use std::collections::HashMap;

use ggmath::prelude::*;

use super::{chunk::{Chunk, CHUNK_SIZE}, world_generator::WorldGenerator};

/// The game world.
/// The world is made up of chunks.
pub struct World {
    chunks: HashMap<Vector3<isize>, Chunk>,
    generator: WorldGenerator,
}

impl World {
    /// Create a new world.
    pub fn new(generator: WorldGenerator) -> Self {
        Self {
            chunks: HashMap::new(),
            generator,
        }
    }

    /// Get the chunk at the given chunk coordinate.
    /// Returns `None` if the chunk does not exist.
    pub fn get_chunk(&self, chunk_coord: Vector3<isize>) -> Option<&Chunk> {
        self.chunks.get(&chunk_coord)
    }

    /// Get a mutable reference to the chunk at the given chunk coordinate.
    /// Returns `None` if the chunk does not exist.
    pub fn get_chunk_mut(&mut self, chunk_coord: Vector3<isize>) -> Option<&mut Chunk> {
        self.chunks.get_mut(&chunk_coord)
    }

    /// Ensure that a chunk exists at the given chunk coordinate.
    /// If the chunk does not exist, it will be created.
    /// Returns a mutable reference to the chunk.
    pub fn ensure_chunk(&mut self, chunk_coord: Vector3<isize>) -> &mut Chunk {
        self.chunks.entry(chunk_coord).or_insert_with(|| Chunk::generate(chunk_coord, &self.generator))
    }

    /// Remove a chunk from the world.
    pub fn remove_chunk(&mut self, position: Vector3<isize>) -> Option<Chunk> {
        self.chunks.remove(&position)
    }
}

/// Trait for converting between world-space coordinates and other spaces.
pub trait WorldSpaceConversion {
    /// Convert a world-space position to a chunk coordinate.
    fn world_to_chunk_coord(self) -> Vector3<isize>;
    /// Convert a world-space position to an in-chunk position.
    fn world_to_chunk(self) -> Self;
}

impl WorldSpaceConversion for Vector3<f32> {
    fn world_to_chunk_coord(self) -> Vector3<isize> {
        self.convert_to().unwrap() / CHUNK_SIZE as isize
    }

    fn world_to_chunk(self) -> Self {
        self % CHUNK_SIZE as f32
    }
}

impl WorldSpaceConversion for Vector3<isize> {
    fn world_to_chunk_coord(self) -> Vector3<isize> {
        self / CHUNK_SIZE as isize
    }

    fn world_to_chunk(self) -> Self {
        self % CHUNK_SIZE as isize
    }
}

/// Trait for converting between chunk coordinates and other spaces.
pub trait ChunkSpaceConversion {
    /// The type returned by `<Self as ChunkSpaceConversion>::chunk_to_world`.
    type ChunkToWorld;

    /// Convert a chunk coordinate to a world-space position.
    /// The position returned is the 0, 0, 0 corner of the chunk.
    fn chunk_coord_to_world(self) -> Vector3<f32>;
    /// Convert an in-chunk position to a world-space position.
    fn chunk_to_world(self, chunk_coord: Vector3<isize>) -> Self::ChunkToWorld;
}

impl ChunkSpaceConversion for Vector3<f32> {
    type ChunkToWorld = Self;

    fn chunk_coord_to_world(self) -> Vector3<f32> {
        self * CHUNK_SIZE as f32
    }

    fn chunk_to_world(self, chunk_coord: Vector3<isize>) -> Self::ChunkToWorld {
        self + (chunk_coord * CHUNK_SIZE as isize).convert_to::<f32>().unwrap()
    }
}

impl ChunkSpaceConversion for Vector3<isize> {
    type ChunkToWorld = Self;

    fn chunk_coord_to_world(self) -> Vector3<f32> {
        (self * CHUNK_SIZE as isize).convert_to::<f32>().unwrap()
    }

    fn chunk_to_world(self, chunk_coord: Vector3<isize>) -> Self::ChunkToWorld {
        self + chunk_coord * CHUNK_SIZE as isize
    }
}

impl ChunkSpaceConversion for Vector3<usize> {
    type ChunkToWorld = Vector3<isize>;

    fn chunk_coord_to_world(self) -> Vector3<f32> {
        (self * CHUNK_SIZE).convert_to::<f32>().unwrap()
    }

    fn chunk_to_world(self, chunk_coord: Vector3<isize>) -> Self::ChunkToWorld {
        self.convert_to().unwrap() + (chunk_coord * CHUNK_SIZE as isize)
    }
}