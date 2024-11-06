use super::buffer::{IndexBuffer, VertexBuffer};

/// A mesh for rendering.
pub struct Mesh {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

impl Mesh {
    /// Create a new `Mesh` with the given buffers.
    pub(crate) fn new(vertex_buffer: VertexBuffer, index_buffer: IndexBuffer) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    /// Get the vertex buffer.
    pub fn vertex_buffer(&self) -> &VertexBuffer {
        &self.vertex_buffer
    }

    /// Get the index buffer.
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }

    /// Get the index count.
    pub fn index_count(&self) -> usize {
        self.index_buffer.len()
    }
}
