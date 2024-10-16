use super::buffer::{IndexBuffer, VertexBuffer};

/// A mesh for rendering.
pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}

impl Mesh {
    /// Create a new `Mesh` with the given buffers.
    pub(crate) fn new(vertex_buffer: VertexBuffer, index_buffer: IndexBuffer) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}
