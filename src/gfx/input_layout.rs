use anyhow::Result;

use super::{buffer::VertexBuffer, vertex_layout::VertexLayout};

// The location the vertex buffer should be bound to.
pub(crate) const VERTEX_BUFFER_LOCATION: u32 = 0;
// The location the instance buffer should be bound to.
pub(crate) const INSTANCE_BUFFER_LOCATION: u32 = 1;

pub struct InputLayout {
    layout: VertexLayout,
    handle: u32,
}

impl !Send for InputLayout {}
impl !Sync for InputLayout {}

impl InputLayout {
    /// Create a new vertex array from the given vertex layout.
    // TODO: Add instancing support.
    pub(crate) fn __from_vertex_layout(layout: VertexLayout) -> Self {
        let mut handle = 0;

        unsafe {
            // Create a vertex array
            gl::CreateVertexArrays(1, &mut handle);

            // Enable the vertex attributes
            let mut offset = 0;
            for (index, input) in layout.inputs().iter().enumerate() {
                gl::EnableVertexArrayAttrib(handle, index as u32);
                gl::VertexArrayAttribFormat(handle, index as u32, input.component_count() as i32, gl::FLOAT, gl::FALSE, offset as u32);
                gl::VertexArrayAttribBinding(handle, index as u32, VERTEX_BUFFER_LOCATION);
                gl::VertexArrayBindingDivisor(handle, index as u32, 0);
                offset += input.component_count() * std::mem::size_of::<f32>();
            }
        }

        Self { layout,  handle }
    }

    /// Get the GL handle.
    pub fn vertex_array_handle(&self) -> u32 {
        self.handle
    }

    /// Get the vertex layout of the buffer.
    pub fn layout(&self) -> &VertexLayout {
        &self.layout
    }

    /// Get the vertex stride.
    pub fn byte_stride(&self) -> usize {
        self.layout.byte_stride()
    }

    /// Validate a vertex buffer for this input layout.
    /// Returns an error if the buffer is not compatible with the layout.
    pub fn validate_buffer(&self, buffer: &VertexBuffer) -> Result<()> {
        if buffer.vertex_layout() != Some(&self.layout) {
            anyhow::bail!("Buffer is not compatible with input layout.");
        }
        Ok(())
    }
}

impl Drop for InputLayout {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
        }
    }
}