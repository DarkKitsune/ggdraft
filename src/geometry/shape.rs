use ggmath::prelude::*;

/// A list of vertices and indices that represent a shape in 3D space using triangles.
pub struct ShapeTriangles {
    /// The positions of the vertices.
    pub(crate) positions: Vec<Vector3<f32>>,
    /// The normals of the vertices.
    pub(crate) normals: Vec<Vector3<f32>>,
    /// The colors of the vertices.
    pub(crate) colors: Vec<Vector4<f32>>,
    /// The indices of the vertices that make up the triangles.
    pub(crate) indices: Vec<u32>,
}

impl ShapeTriangles {
    pub fn append(&mut self, other: &mut ShapeTriangles) {
        let offset = self.positions.len() as u32;
        self.positions.append(&mut other.positions);
        self.normals.append(&mut other.normals);
        self.colors.append(&mut other.colors);
        self.indices
            .append(&mut other.indices.iter().map(|i| i + offset).collect());
    }
}

pub trait ShapeToTriangles {
    /// Converts the shape to a list of triangle vertices.
    fn to_triangles(&self) -> ShapeTriangles;
}

/// A rectangle shape in 3D space.
pub struct Rectangle {
    /// The center of the rectangle.
    pub center: Vector3<f32>,
    /// The forward direction of the rectangle.
    pub forward: Vector3<f32>,
    /// The up direction of the rectangle.
    pub up: Vector3<f32>,
    /// The size of the rectangle.
    pub size: Vector2<f32>,
    /// The color of the rectangle.
    pub color: Vector4<f32>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(
        center: Vector3<f32>,
        forward: Vector3<f32>,
        up: Vector3<f32>,
        size: Vector2<f32>,
        color: Vector4<f32>,
    ) -> Self {
        // Normalize the forward and up vectors.
        let forward = forward.normalized();
        let up = up.normalized();
        Self {
            center,
            forward,
            up,
            size,
            color,
        }
    }

    /// Creates a new rectangle facing the positive Z-axis.
    pub fn new_z(center: Vector3<f32>, size: Vector2<f32>, color: Vector4<f32>) -> Self {
        Self::new(center, Vector3::unit_z(), Vector3::unit_y(), size, color)
    }
}

impl ShapeToTriangles for Rectangle {
    fn to_triangles(&self) -> ShapeTriangles {
        let half_size = self.size / 2.0;
        let forward = self.forward.normalized();
        let up = self.up.normalized();
        let right = forward.cross(&up).normalized();

        let positions = vec![
            self.center + right * half_size.x() + up * half_size.y(),
            self.center + right * half_size.x() - up * half_size.y(),
            self.center - right * half_size.x() - up * half_size.y(),
            self.center - right * half_size.x() + up * half_size.y(),
        ];

        let normals = vec![forward, forward, forward, forward];

        let colors = vec![self.color, self.color, self.color, self.color];

        let indices = vec![0, 1, 2, 0, 2, 3];

        ShapeTriangles {
            positions,
            normals,
            colors,
            indices,
        }
    }
}

/// A box shape in 3D space.
pub struct Box {
    /// The center of the box.
    pub center: Vector3<f32>,
    /// The forward direction of the box.
    pub forward: Vector3<f32>,
    /// The up direction of the box.
    pub up: Vector3<f32>,
    /// The size of the box.
    pub size: Vector3<f32>,
    /// The color of the box.
    pub color: Vector4<f32>,
}

impl Box {
    /// Creates a new box.
    pub fn new(
        center: Vector3<f32>,
        forward: Vector3<f32>,
        up: Vector3<f32>,
        size: Vector3<f32>,
        color: Vector4<f32>,
    ) -> Self {
        // Normalize the forward and up vectors.
        let forward = forward.normalized();
        let up = up.normalized();
        Self {
            center,
            forward,
            up,
            size,
            color,
        }
    }

    /// Creates a new box facing the positive Z-axis.
    pub fn new_z(center: Vector3<f32>, size: Vector3<f32>, color: Vector4<f32>) -> Self {
        Self::new(center, Vector3::unit_z(), Vector3::unit_y(), size, color)
    }
}

impl ShapeToTriangles for Box {
    fn to_triangles(&self) -> ShapeTriangles {
        let half_size = self.size / 2.0;
        let right = self.forward.cross(&self.up);

        let mut front_face = Rectangle::new(
            self.center + self.forward * half_size.z(),
            self.forward,
            self.up,
            vector!(self.size.x(), self.size.y()),
            self.color,
        )
        .to_triangles();

        let mut back_face = Rectangle::new(
            self.center - self.forward * half_size.z(),
            -self.forward,
            self.up,
            vector!(self.size.x(), self.size.y()),
            self.color,
        )
        .to_triangles();

        let mut right_face = Rectangle::new(
            self.center + right * half_size.x(),
            right,
            self.up,
            vector!(self.size.z(), self.size.y()),
            self.color,
        )
        .to_triangles();

        let mut left_face = Rectangle::new(
            self.center - right * half_size.x(),
            -right,
            self.up,
            vector!(self.size.z(), self.size.y()),
            self.color,
        )
        .to_triangles();

        let mut top_face = Rectangle::new(
            self.center + self.up * half_size.y(),
            self.up,
            -self.forward,
            vector!(self.size.x(), self.size.z()),
            self.color,
        )
        .to_triangles();

        let mut bottom_face = Rectangle::new(
            self.center - self.up * half_size.y(),
            -self.up,
            self.forward,
            vector!(self.size.x(), self.size.z()),
            self.color,
        )
        .to_triangles();

        // Append all the faces together.
        front_face.append(&mut back_face);
        front_face.append(&mut right_face);
        front_face.append(&mut left_face);
        front_face.append(&mut top_face);
        front_face.append(&mut bottom_face);

        front_face
    }
}
