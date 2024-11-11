use ggmath::prelude::*;

use crate::geometry::orientation::Orientation;

/// Represents the type of camera.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    /// A perspective camera.
    Perspective {
        /// The field of view in degrees.
        fov: f32,
    },
    /// An orthographic camera.
    Orthographic,
}

impl Default for CameraType {
    fn default() -> Self {
        Self::Perspective { fov: 90.0 }
    }
}

/// A camera that can be passed to a `RenderParameters` to set the view and projection matrices.
#[derive(Debug, Clone)]
pub struct RenderCamera {
    pub orientation: Orientation,
    pub camera_type: CameraType,
    pub near: f32,
    pub far: f32,
    pub zoom: f32,
}

impl RenderCamera {
    /// Create a new perspective camera.
    pub fn perspective(orientation: Orientation, fov: f32, near: f32, far: f32) -> Self {
        Self {
            orientation,
            camera_type: CameraType::Perspective { fov },
            near,
            far,
            zoom: 1.0,
        }
    }

    /// Create a new orthographic camera.
    pub fn orthographic(orientation: Orientation, near: f32, far: f32) -> Self {
        Self {
            orientation,
            camera_type: CameraType::Orthographic,
            near,
            far,
            zoom: 1.0,
        }
    }

    /// Create a new perspective looking at a target position.
    pub fn perspective_looking_at(
        target: Vector3<f32>,
        rotation: Quaternion<f32>,
        distance: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        // Calculate the position of the camera.
        let position = target - rotation.to_matrix() * Vector3::unit_z() * distance;

        // Create the orientation.
        let orientation = Orientation::new(position, rotation, Vector::one());

        // Create the camera.
        Self::perspective(orientation, fov, near, far)
    }

    /// Get the base FOV of the camera.
    /// This will return `None` if the camera is not a perspective camera.
    pub fn base_fov(&self) -> Option<f32> {
        match self.camera_type {
            CameraType::Perspective { fov } => Some(fov),
            CameraType::Orthographic => None,
        }
    }

    /// Get the current FOV of the camera.
    /// This takes into account the zoom level.
    /// This will return `None` if the camera is not a perspective camera.
    pub fn current_fov(&self) -> Option<f32> {
        let base_fov = self.base_fov()?;
        let target_fov = ((base_fov.to_radians() / 2.0).tan() / self.zoom).atan().to_degrees() * 2.0;
        Some(target_fov)
    }

    /// Calculate the view matrix.
    pub fn get_view_matrix(&self) -> Matrix4x4<f32> {
        // Get the position and rotation matrix.
        let position = self.orientation.position();
        let rotation_matrix = self.orientation.rotation().to_matrix();

        // Get the target position and up vector.
        let target = position + rotation_matrix * Vector3::unit_z();
        let up = rotation_matrix * Vector3::unit_y();

        Matrix::new_view(&position, &target, &up)
    }

    /// Calculate the projection matrix.
    /// If the camera is a perspective camera, the resolution is used to calculate the aspect ratio.
    pub fn get_projection_matrix(&self, viewport_size: Vector2<f32>) -> Matrix4x4<f32> {
        match self.camera_type {
            CameraType::Perspective { fov: _ } => {
                // Calculate the projection matrix.
                // Zoom influences the FOV.
                Matrix::new_projection_perspective(
                    self.current_fov().unwrap().to_radians(),
                    viewport_size.x() / viewport_size.y(),
                    self.near,
                    self.far,
                )
            }
            CameraType::Orthographic => {
                // Calculate the projection matrix.
                // Zoom influences the size of the viewport.
                Matrix::new_projection_orthographic(viewport_size / self.zoom, self.near, self.far)
            }
        }
    }
}
