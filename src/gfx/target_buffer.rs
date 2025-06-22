use anyhow::Result;
use ggmath::prelude::*;

use super::{
    input_layout::{InputLayout, _VERTEX_BUFFER_LOCATION},
    mesh::Mesh,
    program::Program,
    render_parameters::RenderParameters,
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
    pub const fn handle(&self) -> u32 {
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

    /// Clear the buffer depth.
    pub fn clear_depth(&self) {
        unsafe {
            // Bind the buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);

            // Clear the buffer.
            gl::ClearDepth(1.0);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            // Unbind the buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Render a mesh to this buffer.
    pub fn render_mesh(
        &self,
        program: &Program,
        input_layout: &InputLayout,
        parameters: &RenderParameters,
        mesh: &Mesh,
    ) -> Result<()> {
        let vertex_buffer = mesh.vertex_buffer();
        let index_buffer = mesh.index_buffer();
        let index_count = mesh.index_count();

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
            // Enable the attributes in the input layout.
            input_layout.__enable_attributes();

            // Bind this target buffer.
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
            gl::BindVertexArray(input_layout.vertex_array_handle());
            gl::BindVertexBuffer(
                _VERTEX_BUFFER_LOCATION,
                vertex_buffer.handle(),
                0,
                input_layout.byte_stride() as i32,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.handle());

            // Use the program.
            gl::UseProgram(program.handle());

            // Use the parameters.
            program.use_parameters(parameters)?;

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
            gl::BindVertexBuffer(_VERTEX_BUFFER_LOCATION, 0, 0, 0);
            gl::BindVertexArray(0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(())
    }

    /// Sets the viewport for rendering.
    /// This will modify the OpenGL viewport globally, so it should be used with care.
    pub(crate) unsafe fn __set_viewport(
        &self,
        center: Vector2<f32>,
        size: Vector2<f32>,
        framebuffer_size: Vector2<u32>,
    ) {
        // Make size absolute.
        let size = vector!(size.x().abs(), size.y().abs());

        // Calculate the viewport position in pixels using the framebuffer size.
        let top_left = center - size / 2.0;
        let x = (top_left.x() * framebuffer_size.x() as f32) as i32;
        let y = (top_left.y() * framebuffer_size.y() as f32) as i32;
        let width = (size.x() * framebuffer_size.x() as f32) as i32;
        let height = (size.y() * framebuffer_size.y() as f32) as i32;

        // Bind the framebuffer.
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
}

impl Default for TargetBuffer {
    fn default() -> Self {
        Self::DEFAULT
    }
}
