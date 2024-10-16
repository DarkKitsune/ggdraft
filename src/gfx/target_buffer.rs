use anyhow::Result;
use ggmath::prelude::*;

use super::{
    input_layout::{InputLayout, VERTEX_BUFFER_LOCATION}, mesh::Mesh, program::Program
};

/// Represents a GL buffer for rendering to.
// TODO: Make it support other types of buffers besides framebuffers.
pub struct TargetBuffer {
    handle: u32,
}

impl !Send for TargetBuffer {}
impl !Sync for TargetBuffer {}

impl TargetBuffer {
    /// The default framebuffer.
    pub const DEFAULT: TargetBuffer = TargetBuffer { handle: 0 };

    /// Get the GL handle.
    /// Returns 0 if this is the default framebuffer.
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Clear the buffer with a color.
    pub fn clear_with_color(&self, color: Vector4<f32>) {
        unsafe {
            // Bind the buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);

            // Clear the buffer.
            gl::ClearColor(color.x(), color.y(), color.z(), color.w());
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Unbind the buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Render triangles to the buffer using the given vertices.
    pub fn render_mesh(
        &self,
        program: &Program,
        mesh_buffers: &Mesh,
        input_layout: &InputLayout,
    ) -> Result<()> {
        let vertex_buffer = &mesh_buffers.vertex_buffer;
        let index_buffer = &mesh_buffers.index_buffer;

        // Get the index count.
        let index_count = index_buffer.len();

        // Return early if index_count == 0.
        if index_count == 0 {
            return Ok(());
        }

        // Validate the index count.
        if index_count > index_buffer.len() {
            anyhow::bail!("Index count is greater than the buffer length.");
        }
        if index_count % 3 != 0 {
            anyhow::bail!("Index count is not a multiple of 3.");
        }

        // Validate the vertex buffer.
        input_layout.validate_buffer(vertex_buffer)?;

        unsafe {
            // Bind this target buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
            gl::BindVertexArray(input_layout.vertex_array_handle());
            gl::BindVertexBuffer(
                VERTEX_BUFFER_LOCATION,
                vertex_buffer.handle(),
                0,
                input_layout.byte_stride() as i32,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());

            // Use the program.
            gl::UseProgram(program.handle());

            // Draw call.
            gl::DrawElements(
                gl::TRIANGLES,
                index_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            // Stop using the program.
            gl::UseProgram(0);

            // Unbind everything.
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexBuffer(VERTEX_BUFFER_LOCATION, 0, 0, 0);
            gl::BindVertexArray(0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(())
    }
}

impl Default for TargetBuffer {
    fn default() -> Self {
        Self::DEFAULT
    }
}
