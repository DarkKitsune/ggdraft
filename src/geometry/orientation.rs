use ggmath::prelude::*;

/// Represents the orientation of an object in 3D space.
pub struct Orientation {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}

impl Orientation {
    pub fn new(position: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Get the position.
    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    /// Get the rotation.
    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    /// Get the scale.
    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    /// Set the position.
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    /// Set the rotation.
    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    /// Set the scale.
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }

    /// Get the transformation matrix.
    pub fn get_transform(&self) -> Matrix4x4<f32> {
        Matrix4x4::new_translation(&self.position)
            * Matrix4x4::new_rotation(&self.rotation)
            * Matrix4x4::new_scale(&self.scale)
    }

    /// Convert a point from local space to world space.
    pub fn local_to_world(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.get_transform() * point
    }

    /// Convert a point from world space to local space.
    pub fn world_to_local(&self, point: Vector3<f32>) -> Vector3<f32> {
        // Invert the scale, rotation, and position.
        let inverted_scale = vector!(
            1.0 / self.scale.x(),
            1.0 / self.scale.y(),
            1.0 / self.scale.z()
        );
        let inverted_rotation = self.rotation.inverted();
        let inverted_position = -self.position;

        // Create the transformation matrices
        let scale = Matrix4x4::new_scale(&inverted_scale);
        let rotation = Matrix4x4::new_rotation(&inverted_rotation);
        let translation = Matrix4x4::new_translation(&inverted_position);

        // Combine the matrices.
        let transform = translation * rotation * scale;

        // Transform the point.
        (transform * vector!(point.x(), point.y(), point.z())).xyz()
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            scale: Vector::one(),
        }
    }
}

impl Into<Matrix4x4<f32>> for Orientation {
    fn into(self) -> Matrix4x4<f32> {
        self.get_transform()
    }
}

impl Into<Matrix4x4<f32>> for &Orientation {
    fn into(self) -> Matrix4x4<f32> {
        self.get_transform()
    }
}

/// Trait for objects that have an orientation in 3D space.
pub trait HasOrientation {
    fn orientation(&self) -> &Orientation;
    fn orientation_mut(&mut self) -> &mut Orientation;

    /// Get the position of the object.
    fn position(&self) -> Vector3<f32> {
        self.orientation().position()
    }

    /// Get the rotation of the object.
    fn rotation(&self) -> Quaternion<f32> {
        self.orientation().rotation()
    }

    /// Get the scale of the object.
    fn scale(&self) -> Vector3<f32> {
        self.orientation().scale()
    }

    /// Set the position of the object.
    fn set_position(&mut self, position: Vector3<f32>) {
        self.orientation_mut().set_position(position);
    }

    /// Set the rotation of the object.
    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.orientation_mut().set_rotation(rotation);
    }

    /// Set the scale of the object.
    fn set_scale(&mut self, scale: Vector3<f32>) {
        self.orientation_mut().set_scale(scale);
    }

    /// Get the transformation matrix of the object.
    fn get_transform(&self) -> Matrix4x4<f32> {
        self.orientation().get_transform()
    }

    /// Convert a point from local space to world space.
    fn local_to_world(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.orientation().local_to_world(point)
    }

    /// Convert a point from world space to local space.
    fn world_to_local(&self, point: Vector3<f32>) -> Vector3<f32> {
        self.orientation().world_to_local(point)
    }
}
