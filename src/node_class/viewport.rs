use ggmath::prelude::*;
use multiverse_ecs::prelude::*;

use crate::gfx::render_camera::RenderCamera;

define_class! {
    /// A viewport renders its children using the given camera and viewport settings.
    pub class Viewport {
        /// The center position of the viewport.
        center: Vector2<f32>,
        /// The size of the viewport.
        size: Vector2<f32>,
        /// The camera used to render the viewport's children.
        camera: RenderCamera,
    }
}

impl Viewport {
    /// Create a new viewport.
    pub fn new(center: Vector2<f32>, size: Vector2<f32>, camera: RenderCamera) -> Self {
        Self {
            center,
            size,
            camera,
        }
    }

    /// Create a new centered viewport that covers the entire target buffer.
    /// The camera is set to a default orthographic camera.
    pub fn new_default() -> Self {
        Self::new(
            vector!(0.5, 0.5),
            vector!(1.0, 1.0),
            RenderCamera::orthographic_centered(vector!(0.0, 0.0, 0.0), 1.0),
        )
    }

    /// Get the center position of the viewport.
    pub const fn center(&self) -> Vector2<f32> {
        self.center
    }

    /// Get the size of the viewport.
    pub const fn size(&self) -> Vector2<f32> {
        self.size
    }

    /// Get the camera used to render the viewport's children.
    pub const fn camera(&self) -> &RenderCamera {
        &self.camera
    }

    /// Get the aspect ratio of the viewport based on the given target buffer size.
    /// This is the width divided by the height.
    pub fn aspect_ratio(&self, target_buffer_size: Vector2<u32>) -> f32 {
        (target_buffer_size.x() as f32 * self.size.x()) / (target_buffer_size.y() as f32 * self.size.y())
    }
}
