use ggmath::prelude::*;

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
        // Use the buffer pointed to by the handle.
        // TODO: Make it support other types of buffers.
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
        }

        // Clear the buffer.
        unsafe {
            gl::ClearColor(color.x(), color.y(), color.z(), color.w());
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Default for TargetBuffer {
    fn default() -> Self {
        Self::DEFAULT
    }
}
